FROM debian:9 as builder

RUN apt-get update
RUN apt-get install -y build-essential curl libarchive-dev libfuse-dev

RUN useradd builder
RUN mkdir -p /home/builder/archivefs
RUN chown -R builder:builder /home/builder

USER builder
ENV HOME=/home/builder

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="${HOME}/.cargo/bin:${PATH}"

WORKDIR /home/builder/archivefs

ADD Cargo.lock Cargo.toml ./
ADD src src/
RUN cargo update

ADD include include/
ADD build.rs ./
ADD Makefile ./
RUN make

RUN mkdir -p "dist/usr/local/bin"
RUN mkdir -p "dist/usr/local/opt/archivefs/lib"
RUN mkdir -p "dist/DEBIAN"

ADD debian/control dist/DEBIAN
RUN cp archivefs dist/usr/local/bin
RUN cp target/release/libarchivefs.so dist/usr/local/opt/archivefs/lib

WORKDIR /output
ENTRYPOINT ["dpkg-deb", "--build", "/home/builder/archivefs/dist", "/output"]
