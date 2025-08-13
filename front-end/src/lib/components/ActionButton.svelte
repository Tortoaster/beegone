<script lang="ts">
  import type { SVGAttributes } from "svelte/elements"
  import { fly, type FlyParams } from "svelte/transition"
  import type { Action, Piece } from "beegone"
  import ActionIcon from "$lib/icons/ActionIcon.svelte"

  export interface ActionButtonProps extends SVGAttributes<SVGElement> {
    action: Action
    x: number
    y: number
    size: number
    dx: number
    dy: number
    piece: Piece | undefined
    delay: number
    onaction: (action: Action) => void
  }

  const {
    action,
    x,
    y,
    size,
    piece,
    dx,
    dy,
    delay,
    onclick,
    onaction,
    class: classValue,
    ...props
  }: ActionButtonProps = $props()

  const inTransition: FlyParams = $derived({
    x: dx,
    y: dy,
    delay: (Math.atan2(-dy, -dx) + Math.PI) * 30 + delay,
    duration: 200,
  })
  const outTransition: FlyParams = $derived({
    x: -dx,
    y: -dy,
    duration: 200,
  })
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  x={x - size / 2}
  y={y - size / 2}
  width={size}
  height={size}
  class={["group cursor-pointer", classValue]}
  onclick={(e) => {
    onclick?.(e)
    onaction(action)
  }}
  in:fly={inTransition}
  out:fly={outTransition}
  {...props}
>
  <circle
    class="transition-colors fill-accent-light group-hover:fill-accent"
    cx="50%"
    cy="50%"
    r="50%"
  />
  <ActionIcon
    {action}
    {piece}
    class="transition-colors fill-accent-dark group-hover:fill-white"
    x="25%"
    y="25%"
    width="50%"
    height="50%"
  />
</svg>
