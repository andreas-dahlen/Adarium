import { createSignal } from 'solid-js'
import styles from './App.module.css'
import { calculate_potatoes } from 'rust-wasm'

export default function App() {
  const [amount, setAmount] = createSignal<number>(10)
  const [result, setResult] = createSignal<number>()

  const callWasm = () => {
    const result = calculate_potatoes(amount())
    setResult(result)
  }

  return (
    <main class={styles.app}>
      <header>
        <h1>Rust WASM Playground</h1>
        <p>Potato calculations powered by WebAssembly.</p>
      </header>

      <section class={styles.calculator}>
        <label for="amount">Amount of potatoes</label>

        <input
          id="amount"
          type="number"
          min="0"
          value={amount()}
          onInput={(event) => setAmount(Number(event.currentTarget.value))}
        />

        <button
          onclick={callWasm}>
          Calculate
        </button>

        <div class={styles.result}>
          Result: {result() ?? 'Waiting for calculation...'}
        </div>
      </section>
    </main>
  )
}