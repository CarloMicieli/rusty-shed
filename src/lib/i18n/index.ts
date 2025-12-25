import { register, init, getLocaleFromNavigator } from 'svelte-i18n';

register('en', () => import('./en.json'));
register('it', () => import('./it.json'));

init({
    fallbackLocale: 'en',
    initialLocale: getLocaleFromNavigator(),
});
