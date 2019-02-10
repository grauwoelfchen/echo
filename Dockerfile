FROM grauwoelfchen/rust:latest as app

RUN mkdir /echo
COPY . /echo
WORKDIR /echo
RUN make build:release


FROM gentoo/portage:latest as portage
FROM gentoo/stage3-amd64-nomultilib:latest

COPY --from=portage /usr/portage /usr/portage

RUN mkdir /srv
COPY --from=app /echo/target/release/echo /srv/echo
WORKDIR /srv
CMD ["./echo"]
