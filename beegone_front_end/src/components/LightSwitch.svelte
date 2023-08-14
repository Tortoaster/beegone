<script lang="ts">
	import { onMount } from 'svelte';

	const STORAGE_KEY = 'theme';
	const MEDIA_QUERY = '(prefers-color-scheme: dark)';

	enum Theme {
		Dark = 'dark',
		Light = 'light',
	}

	const defaultDark = () => window.matchMedia(MEDIA_QUERY).matches;

	const currentTheme = () =>
		(localStorage.getItem(STORAGE_KEY) as Theme) ?? (defaultDark() ? Theme.Dark : Theme.Light);

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
		window.matchMedia(MEDIA_QUERY).addEventListener('change', applyCurrentTheme);
		return () => window.matchMedia(MEDIA_QUERY).removeEventListener('change', applyCurrentTheme);
	});
</script>

<input type="checkbox" on:click={toggleTheme} />
