FROM node:20 AS build
COPY package*.json .
COPY .npmrc .
RUN npm ci
COPY *.config.ts .
COPY *.config.js .
COPY ./static ./static
COPY ./src ./src
RUN npm run build
ARG NODE_ENV=production
ENV NODE_ENV $NODE_ENV
CMD ["node", "build"]