FROM fedora:latest

RUN dnf update -y && dnf install -y \
    git \
    curl \
    gcc \
    make \
    bzip2 \
    pypy \
    pypy3 \
    libwayland-client \
    libwayland-cursor \
    libwayland-egl \
    wayland-devel \
    libxkbcommon \
    mesa-libEGL \
    mesa-libGL \
    mesa-dri-drivers \
    vulkan-loader \
    && dnf clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    /root/.cargo/bin/rustup default stable

ENV PATH="/root/.cargo/bin:${PATH}"
