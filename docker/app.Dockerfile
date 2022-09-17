FROM node:18-alpine3.15 as base

ARG PUBLIC_API_URL
ARG PUBLIC_APP_URL
ARG CHAT_BOT_USERNAME

ENV PUBLIC_API_URL=${PUBLIC_API_URL}
ENV PUBLIC_APP_URL=${PUBLIC_APP_URL}}
ENV CHAT_BOT_USERNAME=${CHAT_BOT_USERNAME}

RUN corepack enable

RUN echo $PUBLIC_APP_URL

FROM base as build

WORKDIR /app

COPY ./app/package.json ./app/.npmrc ./app/pnpm-lock.yaml ./

RUN pnpm i

FROM base as application

WORKDIR /app

COPY --from=build /app/node_modules ./node_modules

COPY ./app ./

RUN pnpm run build

ENTRYPOINT ["node", "build/index.js"]
