FROM rust:1.71-slim AS wasm_tools
RUN cargo install typeshare-cli
RUN cargo install wasm-pack

FROM wasm_tools AS wasm
COPY . /app
WORKDIR /app
RUN typeshare -l typescript -o wasm/types.d.ts beegone
RUN wasm-pack build --release --out-dir ../wasm --scope beegone beegone

FROM node:20-slim AS base
RUN npm install -g pnpm
COPY --from=wasm /app /app
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
