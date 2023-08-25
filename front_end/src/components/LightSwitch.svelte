<script lang="ts">
	import { onMount } from 'svelte';

	const STORAGE_KEY = 'theme';

	enum Theme {
		Dark = 'dark',
		Light = 'light',
	}

	const currentTheme = () => (localStorage.getItem(STORAGE_KEY) as Theme) ?? Theme.Light;

	const toggleTheme = () => {
		const toggledTheme = currentTheme() === Theme.Dark ? Theme.Light : Theme.Dark;
		localStorage.setItem(STORAGE_KEY, toggledTheme);
		applyTheme(toggledTheme);
	};

	const applyTheme = (theme: Theme) => {
		if (theme === Theme.Dark) {
			document.documentElement.classList.add(Theme.Dark);
		} else {
			document.documentElement.classList.remove(Theme.Dark);
		}
	};

	const applyCurrentTheme = () => applyTheme(currentTheme());

	onMount(() => {
		applyCurrentTheme();
	});
</script>

<button class="p-2" on:click={toggleTheme}>
	<svg class="w-8 h-8" viewBox="-4 -4 8 8">
		<path
			class="transition-colors duration-300 fill-amber-600 dark:fill-slate-600"
			d="M-1,-2 l4,0 a1,1,0,0,1,1,1 l0,4 a1,1,0,0,1,-1,1 l-4,0 a1,1,0,0,1,-1,-1 l0,-4 a1,1,0,0,1,1,-1z"
		/>
		<path
			class="transition-colors duration-300 fill-amber-400 dark:fill-slate-400"
			d="M-3,-4 l4,0 a1,1,0,0,1,1,1 l0,4 a1,1,0,0,1,-1,1 l-4,0 a1,1,0,0,1,-1,-1 l0,-4 a1,1,0,0,1,1,-1z"
		/>
	</svg>
</button>
