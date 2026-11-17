{{#include ../include/header020.md}}

# Messages previously known as Events

Relevant official examples:
[`message`][example::code::messages].

---

Send data between systems! Let your [systems][cb::system] communicate with each other!

Like [resources][cb::res] or [components][cb::component], messages are
simple Rust `struct`s or `enum`s. When creating a new message type, derive
the [`Message`] trait.

Then, any [system][cb::system] can send (broadcast) values of that type,
and any system can receive those messages.

 - To send messages, use an [`MessageWriter<T>`].
 - To receive messages, use an [`MessageReader<T>`].
 - To send and receive messages, use an [`MessageMutator<T>`].

Every reader tracks the message it has read independently, so you can handle
the same messages from multiple [systems][cb::system].

```rust,no_run,noplayground
#[derive(Message)]
struct LevelUpMessage(Entity);

fn player_level_up(
    mut ev_levelup: MessageWriter<LevelUpMessage>,
    query: Query<(Entity, &PlayerXp)>,
) {
    for (entity, xp) in query.iter() {
        if xp.0 > 1000 {
            ev_levelup.send(LevelUpMessage(entity));
        }
    }
}

fn debug_levelups(
    mut ev_levelup: MessageReader<LevelUpMessage>,
) {
    for ev in ev_levelup.read() {
        eprintln!("Entity {:?} leveled up!", ev.0);
    }
}
```

You need to register your custom message types via the [app builder][cb::app]:

```rust,no_run,noplayground
app.add_message::<LevelUpMessage>();
```

## Usage Advice

Messages should be your go-to data flow tool. As messages can be sent from any
[system][cb::system] and received by multiple systems, they are *extremely*
versatile.

Messages can be a very useful layer of abstraction. They allow you to decouple
things, so you can separate different functionality and more easily reason
about which [system][cb::system] is responsible for what.

You can imagine how, even in the simple "player level up" example shown above,
using messages would allow us to easily extend our hypothetical game with more
functionality. If we wanted to display a fancy level-up effect or animation,
update UI, or anything else, we can just add more systems that read the messages
and do their respective things. If the `player_level_up` system had simply
checked the player XP and managed the player level directly, without going via
messages, it would be unwieldy for future development of the game.

## How it all works

When you register an message type, Bevy will create an [`Messages<T>`]
[resource][cb::res], which acts as the backing storage for the message queue. Bevy
also adds an "message maintenance" [system][cb::system] to clear messages periodically,
prmessageing them from accumulating and using up memory.

Bevy ensures that messages are kept around for at least two frame update cycles,
or two [fixed timestep][cb::fixedtimestep] cycles, whichever is longer. After
that, they are silently dropped. This gives your systems enough opportunity
to handle them, assuming your systems are running all the time. Beware when
adding [run conditions][cb::rc] to your systems, as you might miss some messages
when your systems are not running!

If you don't like this, [you can have manual control over when messages are
cleared][cb::message-manual] (at the risk of leaking / wasting memory if you
forget to clear them).

The [`MessageWriter<T>`] system parameter is just syntax sugar for mutably
accessing the [`Messages<T>`] [resource][cb::res] to add messages to the queue. The
[`MessageReader<T>`] is a little more complex: it accesses the messages storage
immutably, but also stores an integer counter to keep track of how many messages
you have read. This is why it also needs the `mut` keyword.

[`Messages<T>`] itself is internally implemented using simple [`Vec`]s. Sending
messages is equivalent to just pushing to a [`Vec`]. It is very fast,
low overhead. Messages are often the most performant way to implement things
in Bevy, better than using [change detection][cb::change-detection].

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the
receiving system before the sending system. The receiving system will only
get a chance to receive the messages the next time it runs. If you need to
ensure that messages are handled on the same frame, you can use [explicit
system ordering][cb::system-order].

If your systems have [run conditions][cb::rc], beware that they might miss
some messages when they are not running! If your system does not check for messages
at least once every other frame or [fixed timestep][cb::fixedtimestep], the
messages will be lost.

If you want messages to persist for longer than that, you can [implement a
custom cleanup/management strategy][cb::message-manual]. However, you can
only do this for your own message types. There is no solution for Bevy's
[built-in][builtins::message] types.
