FROM rust:1.61.0
# Set application working directory
WORKDIR /usr/src/app
# Copy All Data
COPY . .
# Install application
RUN cargo install --path .
CMD ["PRKSChingService"]