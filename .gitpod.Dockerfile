FROM gitpod/workspace-full

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq \
        qemu \
    && sudo rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/home/gitpod/.cargo