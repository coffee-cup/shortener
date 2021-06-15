FROM rustlang/rust:nightly

ARG PORT

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=staging

WORKDIR /app
COPY . .

RUN yarn
RUN yarn build

CMD ROCKET_PORT=$PORT cargo run
