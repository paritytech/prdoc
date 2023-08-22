FROM docker.io/paritytech/ci-linux:production as builder

ARG PROFILE=production
WORKDIR /app

COPY . .

RUN cargo build --profile ${PROFILE} --bins

# MAIN IMAGE FOR PEOPLE TO PULL --- small one#
FROM docker.io/parity/base-bin

USER root
ARG PROFILE=production
WORKDIR /usr/local/bin

COPY --from=builder /app/target/$PROFILE/prdoc /usr/local/bin

USER parity
ENTRYPOINT [ "prdoc" ]
