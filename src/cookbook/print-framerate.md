{{#include ../include/header020.md}}

# Show Framerate

You can use Bevy's builtin diagnostics to measure framerate (FPS), for
monitoring performance.

To enable it, add Bevy's bevy render_dev_tools to your cargo toml.
Then add the FPS plugin to your app [app][cb::app].

```rust,no_run,noplayground
{{#include ../code020/examples/print_framerate.rs:setup}}
```



Here is the full fps counter [example][example::code::fps] viewable in the [browser][example::fps]

<details>
  <summary>
  <code>Code Example (Long):</code>
  </summary>

```rust,no_run,noplayground
{{#include ../code020/examples/print_framerate.rs}}
```

</details>


## Print to Console / Log

The simplest way to use it is to print the diagnostics to the console
([log][cb::log]). If you want to only do it in dev builds, you can add
a conditional-compilation attribute.

[example][example::log]
[example code][example::code::log]


