FROM rustlang/rust:nightly

# Install node
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs \
    && npm i -g yarn

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=production

WORKDIR /app
COPY . .

RUN yarn
RUN yarn build

CMD ROCKET_PORT=$PORT cargo run
