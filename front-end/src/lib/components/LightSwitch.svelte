<script lang="ts">
	import { onMount } from 'svelte';

	const STORAGE_KEY = 'theme';

	type Theme = 'dark' | 'light';

	const currentTheme = () => (localStorage.getItem(STORAGE_KEY) as Theme) ?? 'light';

	const toggleTheme = () => {
		const toggledTheme = currentTheme() === 'dark' ? 'light' : 'dark';
		localStorage.setItem(STORAGE_KEY, toggledTheme);
		applyTheme(toggledTheme);
	};

	const applyTheme = (theme: Theme) => {
		if (theme === 'dark') {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	};

	onMount(() => {
		applyTheme(currentTheme());
	});
</script>

<button class="p-2" on:click={toggleTheme}>
	<svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em">
		<defs>
			<filter id="icon-color">
				<feFlood class="transition-all duration-300 flood-amber-700 dark:flood-slate-300" />
				<feComposite operator="in" in2="SourceGraphic" />
			</filter>
		</defs>
		<image xlink:href="/theme.svg" width="100%" height="100%" filter="url(#icon-color)" />
	</svg>
</button>
