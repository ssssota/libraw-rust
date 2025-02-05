import { render } from 'preact'
import { App } from './app.tsx'

const parent = document.getElementById('app')
parent && render(<App />, parent)
