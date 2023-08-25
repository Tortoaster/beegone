<script lang="ts">
	import { onMount } from 'svelte';
	import ThemeIcon from '../icons/ui/ThemeIcon.svelte';

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

	onMount(() => {
		applyTheme(currentTheme());
	});
</script>

<button class="p-2" on:click={toggleTheme}>
	<ThemeIcon
		width="2em"
		height="2em"
		topClass="transition-colors duration-300 fill-amber-600 dark:fill-slate-600"
	/>
</button>
