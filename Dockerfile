FROM node:20-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run -r build

FROM base AS beegone
# beegone has no dependencies
# COPY --from=prod-deps /app/wasm/node_modules/ wasm/node_modules
# beegone is a library
# COPY --from=build /app/wasm/build wasm/build

# front_end depends on beegone
FROM beegone AS front_end
COPY --from=prod-deps /app/front_end/node_modules/ front_end/node_modules
COPY --from=build /app/front_end/build front_end/build
WORKDIR front_end
ENV PORT=8000
EXPOSE 8000
CMD [ "pnpm", "start" ]
