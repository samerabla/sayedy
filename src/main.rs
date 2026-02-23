use leptos::ev::MouseEvent;
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <App/>
        }
    });
}

#[component]
pub fn App1() -> impl IntoView {
    let (net, set_net) = signal(0.0);
    let total = move || (net.get() / 81.0) * 100.0;
    view! {
        <div>
            <div>
                <input
                    on:input:target=move |e| {
                        set_net.set(e.target().value().parse().unwrap_or_default())
                    }
                    prop:value=net
                />
            </div>
            <div>Total: {move || format!("{:.2}", total())}</div>
            <pre>
                Netto: {move || format!("{:.2}", net.get())}
                MWST: {move || format!("{:.2}", net.get() * 0.19)}
                -----------------------------------
                Brutto: {move || format!("{:.2}", total())}
            </pre>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (net, set_net) = signal(0.0f64);
    let brutto = move || (net.get() / 81.0) * 100.0;

    let mwst = move || brutto() - net.get();

    view! {
        <style>
            {r#"
            @import url('https://fonts.googleapis.com/css2?family=Syne:wght@400;700;800&family=JetBrains+Mono:wght@400;600&display=swap');

            *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

            :root {
                --bg: #F4F1EB;
                --surface: #FFFFFF;
                --ink: #111111;
                --ink-muted: #6B6B6B;
                --accent: #FF3B00;
                --accent-light: #FFF0EC;
                --border: #E0DDD6;
                --mono: 'JetBrains Mono', monospace;
                --display: 'Syne', sans-serif;
            }

            body {
                background: var(--bg);
                min-height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                font-family: var(--display);
                padding: 2rem;
                background-image:
                    radial-gradient(circle at 15% 85%, rgba(255,59,0,0.06) 0%, transparent 50%),
                    radial-gradient(circle at 85% 10%, rgba(255,59,0,0.04) 0%, transparent 45%);
            }

            .card {
                background: var(--surface);
                border: 2px solid var(--ink);
                width: 100%;
                max-width: 460px;
                box-shadow: 8px 8px 0px var(--ink);
                overflow: hidden;
                animation: slideUp 0.5s cubic-bezier(0.22, 1, 0.36, 1) both;
            }

            @keyframes slideUp {
                from { opacity: 0; transform: translateY(24px); }
                to   { opacity: 1; transform: translateY(0); }
            }

            .card-header {
                background: var(--ink);
                color: #fff;
                padding: 1.5rem 2rem;
                display: flex;
                align-items: baseline;
                gap: 0.75rem;
            }

            .card-header h1 {
                font-size: 1.5rem;
                font-weight: 800;
                letter-spacing: -0.03em;
                text-transform: uppercase;
            }

            .card-header .badge {
                font-family: var(--mono);
                font-size: 0.65rem;
                font-weight: 600;
                background: var(--accent);
                color: #fff;
                padding: 0.2rem 0.5rem;
                letter-spacing: 0.08em;
                border-radius: 2px;
                text-transform: uppercase;
            }

            .card-body {
                padding: 2rem;
            }

            .input-group {
                margin-bottom: 2rem;
            }

            .input-label {
                display: block;
                font-size: 0.7rem;
                font-weight: 700;
                letter-spacing: 0.12em;
                text-transform: uppercase;
                color: var(--ink-muted);
                margin-bottom: 0.6rem;
            }

            .input-wrapper {
                position: relative;
                display: flex;
                align-items: center;
                border: 2px solid var(--ink);
                transition: box-shadow 0.15s ease;
            }

            .input-wrapper:focus-within {
                box-shadow: 4px 4px 0px var(--accent);
            }

            .currency-sym {
                padding: 0 1rem;
                font-family: var(--mono);
                font-size: 1.1rem;
                font-weight: 600;
                color: var(--ink-muted);
                border-right: 2px solid var(--border);
                background: #FAFAFA;
                line-height: 3.2rem;
                user-select: none;
            }

            .net-input {
                flex: 1;
                border: none;
                outline: none;
                padding: 0 1rem;
                font-family: var(--mono);
                font-size: 1.4rem;
                font-weight: 600;
                color: var(--ink);
                background: transparent;
                height: 3.2rem;
                -moz-appearance: textfield;
            }

            .net-input::-webkit-inner-spin-button,
            .net-input::-webkit-outer-spin-button { -webkit-appearance: none; }

            .breakdown {
                border: 2px solid var(--border);
                background: var(--bg);
            }

            .breakdown-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 0.9rem 1.25rem;
                border-bottom: 1px solid var(--border);
            }

            .divider-line {
                border: none;
                border-top: 1px dashed var(--border);
                margin: 0 1.25rem;
            }

            .row-label {
                font-family: var(--mono);
                font-size: 0.72rem;
                font-weight: 600;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                color: var(--ink-muted);
            }

            .row-value {
                font-family: var(--mono);
                font-size: 1rem;
                font-weight: 600;
                color: var(--ink);
            }

            .row-value.tax {
                color: var(--accent);
            }

            .breakdown-total {
                background: var(--ink);
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 1.1rem 1.25rem;
            }

            .total-label {
                font-size: 0.75rem;
                font-weight: 700;
                letter-spacing: 0.14em;
                text-transform: uppercase;
                color: rgba(255,255,255,0.6);
                font-family: var(--mono);
            }

            .total-value {
                font-family: var(--mono);
                font-size: 1.5rem;
                font-weight: 600;
                color: #fff;
                letter-spacing: -0.02em;
            }

            .footer-note {
                margin-top: 1.25rem;
                font-family: var(--mono);
                font-size: 0.65rem;
                color: var(--ink-muted);
                letter-spacing: 0.06em;
                text-align: right;
            }
            "#}
        </style>

        <div class="card">
            <div class="card-header">
                <h1>"Hussen Bamerny"</h1>
            </div>

            <div class="card-body">
                <div class="input-group">
                    <label class="input-label">"Nettobetrag eingeben"</label>
                    <div class="input-wrapper">
                        <span class="currency-sym">"€"</span>
                        <input
                            class="net-input"
                            type="number"
                            step="1"
                            min="0"

                            on:input:target=move |e| {
                                set_net.set(e.target().value().parse().unwrap_or_default())
                            }
                            prop:value=move || net.get()
                        />
                    </div>
                </div>

                <div class="breakdown">
                    <div class="breakdown-row">
                        <span class="row-label">"Netto"</span>
                        <span class="row-value">{move || format!("€ {:.2}", net.get())}</span>
                    </div>
                    <hr class="divider-line"/>
                    <div class="breakdown-row">
                        <span class="row-label">"MwSt (19%)"</span>
                        <span class="row-value tax">{move || format!("€ {:.2}", mwst())}</span>
                    </div>
                    <div class="breakdown-total">
                        <span class="total-label">"Brutto"</span>
                        <span class="total-value">{move || format!("€ {:.2}", brutto())}</span>
                    </div>
                </div>

                <p class="footer-note">"سيدي و مولاي"</p>
            </div>
        </div>
    }
}
