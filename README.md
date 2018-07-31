# Rust on Platform.sh

Platform.sh doesn't officially support Rust, but that doesn't mean you can't install the compiler in the build container.
This project contains both backend code containing a basic [Rocket](https://rocket.rs) application and a webasm frontend built by `cargo web`.

Just commit this to a new project and Platform.sh will take care of the rest.
