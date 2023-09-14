####################################################################################################
## Builder
####################################################################################################
FROM rust:bullseye AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=sharpitdev
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /sharpitdev

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

RUN chmod +x /sharpitdev/target/release/dummy-api

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /sharpitdev

# Copy our build
COPY --from=builder /sharpitdev/target/release/dummy-api ./

# Use an unprivileged user.
USER sharpitdev:sharpitdev

CMD ["/sharpitdev/dummy-api"]
