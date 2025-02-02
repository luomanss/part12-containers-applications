FROM node:20-alpine AS build-stage

WORKDIR /usr/src/app

COPY . .

RUN npm ci

ENV VITE_BACKEND_URL=http://localhost:8080/api

RUN npm run build

FROM httpd:alpine

COPY --from=build-stage /usr/src/app/dist /usr/local/apache2/htdocs/