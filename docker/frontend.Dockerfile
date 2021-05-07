FROM node:alpine AS deps

RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package.json yarn.lock ./
RUN curl -f https://get.pnpm.io/v6.js | node - add --global pnpm

RUN pnpm install


# Rebuild the source code only when needed
FROM node:alpine AS builder
WORKDIR /app
COPY fronted/ .
COPY --from=deps /app/node_modules ./node_modules
RUN pnpm build

# Production image, copy all the files and run next
FROM node:alpine AS runner
WORKDIR /app

ENV NODE_ENV production

RUN addgroup -g 1001 -S node
RUN adduser -S ui -u 1001

# You only need to copy next.config.js if you are NOT using the default configuration
COPY --from=builder /app/public ./public
COPY --from=builder --chown=ui:node /app/.next ./.next
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./package.json

USER ui

EXPOSE 3000

CMD ["pnpm", "start"]
