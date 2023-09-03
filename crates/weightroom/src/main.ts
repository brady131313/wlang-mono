import { start } from 'wlang-web'
import './app.css'
import App from './App.svelte'

start()

const app = new App({
    target: document.getElementById('app')!,
})

export default app
