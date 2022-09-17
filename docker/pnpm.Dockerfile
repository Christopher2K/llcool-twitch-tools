FROM node:18-alpine3.15 as base

RUN corepack enable

WORKDIR /app

