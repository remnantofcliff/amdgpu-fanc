#!/bin/bash

bindgen src/signals/signal_handling.hpp \
    --enable-cxx-namespaces             \
    --respect-cxx-access-specs          \
    -o src/signals/bindings.rs
