FROM node:20-alpine

RUN mkdir -p /usr/src/app
RUN chown node:node /usr/src/app
WORKDIR /usr/src/app

USER node

COPY --chown=node:node . .

RUN npm ci

EXPOSE 3000

CMD ["npm", "start"]