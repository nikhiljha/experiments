<script>
	import { quintOut } from "svelte/easing";
	import { fade, draw } from "svelte/transition";
	import { shrink } from "./transitions.js";
	import { onMount } from "svelte";
	let visible = true;
	onMount(() => (visible = false));
</script>

<!-- obligatory marquee -->
<marquee>cool website</marquee>
<svg
	xmlns="http://www.w3.org/2000/svg"
	xmlns:xlink="http://www.w3.org/1999/xlink"
	viewBox="0 0 333.08 502.15"
	height="420px">
	<defs>
		<style>
			.cls-1 {
				fill: none;
				stroke-miterlimit: 10;
				stroke-width: 1.5px;
				stroke: url(#linear-gradient);
			}

			.cls-2 {
				fill: url(#Orange_Yellow);
			}
		</style>
		<linearGradient
			id="linear-gradient"
			x1="259.97"
			y1="448.73"
			x2="42.04"
			y2="71.27"
			gradientUnits="userSpaceOnUse">
			<stop offset="0" stop-color="#fff33b" />
			<stop offset="0.04" stop-color="#fee72e" />
			<stop offset="0.12" stop-color="#fed51b" />
			<stop offset="0.2" stop-color="#fdca10" />
			<stop offset="0.28" stop-color="#fdc70c" />
			<stop offset="0.67" stop-color="#f3903f" />
			<stop offset="0.89" stop-color="#ed683c" />
			<stop offset="1" stop-color="#e93e3a" />
		</linearGradient>
		<linearGradient
			id="Orange_Yellow"
			x1="258.9"
			y1="446.81"
			x2="43.1"
			y2="73.05"
			gradientUnits="userSpaceOnUse">
			<stop offset="0" stop-color="#fff33b" />
			<stop offset="0.04" stop-color="#fee72e" />
			<stop offset="0.12" stop-color="#fed51b" />
			<stop offset="0.2" stop-color="#fdca10" />
			<stop offset="0.28" stop-color="#fdc70c" />
			<stop offset="0.67" stop-color="#f3903f" />
			<stop offset="0.89" stop-color="#ed683c" />
			<stop offset="1" stop-color="#e93e3a" />
		</linearGradient>
	</defs>
	<g id="Layer_2" data-name="Layer 2">
		{#if !visible}
			<g id="trace">
				<path
					class="cls-1"
					in:draw={{ duration: 2000 }}
					out:shrink={{ duration: 400 }}
					on:introstart={() =>
						setTimeout(() => {
							visible = !visible;
						}, 600)}
					d="M125.74,360.65l41.57,24.41,40.9-24.41-23,77.5L147.73,438Zm22.73,80.27L167,499.59l17.47-58.67Zm13.35-277.65h9.44l-4.72-33.75Zm51.24-82-45-78.47V124.07Zm-43.32,45.3,68.65.09-23.77-43.1Zm-.86,2.93,5.21,34,34,20.14,31-54.12Zm72.21,1.7-30.28,53.66,43.77,25.28,11-34.64Zm22.43,84.94,1,38.6h47.42l-44.54-76.52-10.51,33.39ZM290.68,341l41.49-52.71-18.44-30.74H264.4l.28,68.6ZM172.84,489.73l32.54-48.81H187.72Zm15.58-17.35,25.78-31.46h-5.43ZM268.16,332l-5.2-3.47L218.37,356l-10.8,18.27-19.2,63.9H207ZM210.1,438.15h6.06l72.75-94.73-18.11-9.95ZM165,124.07V2.82L120,81.29ZM118.45,83.58l-23.77,43.1,68.65-.09ZM94,129.52l31,54.12,34-20.14,5.2-34Zm-26.52,46,11,34.64,43.77-25.28L92,131.22Zm8.71,36.11L65.72,178.24,21.18,254.76H68.61l1-38.6ZM68.4,326.18l.27-68.6H19.35L.91,288.32,42.39,341Zm77,114.74H127.7l32.53,48.81Zm-21.06,0h-5.42l25.78,31.46Zm1.75-2.77H144.7l-19.19-63.9L114.71,356,70.12,328.49,64.91,332ZM62.27,333.47l-18.11,9.95,72.76,94.73H123Z" />
			</g>
		{/if}

		{#if visible}
			<g id="trace-fill">
				<path
					class="cls-2"
					in:fade={{ duration: 400, easing: quintOut }}
					d="M125.74,360.65l41.57,24.41,40.9-24.41-23,77.5L147.73,438Zm22.73,80.27L167,499.59l17.47-58.67Zm13.35-277.65h9.44l-4.72-30Zm51.24-82-45-78.47V124.07Zm-43.32,45.3,68.65.09-23.77-43.1Zm-.86,2.93,5.21,34,34,20.14,31-54.12Zm72.21,1.7-30.28,53.66,43.77,25.28,11-34.64Zm22.43,84.94,1,38.6h47.42l-44.54-76.52-10.51,33.39ZM290.68,341l41.49-52.71-18.44-30.74H264.4l.28,68.6ZM172.84,489.73l32.54-48.81H187.72Zm15.58-17.35,25.78-31.46h-5.43ZM268.16,332l-5.2-3.47L218.37,356l-10.8,18.27-19.2,63.9H207ZM210.1,438.15h6.06l72.75-94.73-18.11-9.95ZM165,124.07V2.82L120,81.29ZM118.45,83.58l-23.77,43.1,68.65-.09ZM94,129.52l31,54.12,34-20.14,5.2-34Zm-26.52,46,11,34.64,43.77-25.28L92,131.22Zm8.71,36.11L65.72,178.24,21.18,254.76H68.61l1-38.6ZM68.4,326.18l.27-68.6H19.35L.91,288.32,42.39,341Zm77,114.74H127.7l32.53,48.81Zm-21.06,0h-5.42l25.78,31.46Zm1.75-2.77H144.7l-19.19-63.9L114.71,356,70.12,328.49,64.91,332ZM62.27,333.47l-18.11,9.95,72.76,94.73H123Z" />
			</g>
		{/if}
	</g>
</svg>
