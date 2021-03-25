FROM node:15
EXPOSE 3000
COPY . .
RUN yarn build
CMD ["yarn", "start", "-H", "0.0.0.0"]

