# Gentoo Linux container with Rust (latest)
# https://github.com/grauwoelfchen/portolan/blob/master/rust/latest/Dockerfile
FROM grauwoelfchen/rust:latest as app

RUN mkdir /echo
COPY . /echo
WORKDIR /echo
RUN make build:release


FROM gentoo/portage:latest as portage
FROM gentoo/stage3-amd64-nomultilib:latest

COPY --from=portage /usr/portage /usr/portage

ARG HOST
ARG PORT

ENV HOST=$HOST
ENV PORT=$PORT

RUN mkdir /srv
COPY --from=app /echo/target/release/echo /srv/echo
WORKDIR /srv
CMD ["./echo"]
