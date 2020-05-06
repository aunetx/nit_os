FROM gitpod/workspace-full
                    
USER gitpod

# Install custom tools, runtime, etc. using apt-get
# For example, the command below would install "bastet" - a command line tetris clone:
#
# RUN sudo apt-get -q update && #     sudo apt-get install -yq bastet && #     sudo rm -rf /var/lib/apt/lists/*
#
# More information: https://www.gitpod.io/docs/config-docker/

RUN sudo apt-get -q update \
    && sudo apt-get install qemu \
    && sudo rm -rf /var/lib/apt/lists/*

RUN rustup toolchain install nightly \
    && rustup default nightly \
    && rustup component add rust-src llvm-tools-preview

# TODO download prebuilt package
RUN cargo install cargo-make cargo-xbuild bootimage