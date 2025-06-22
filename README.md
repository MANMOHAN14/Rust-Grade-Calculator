
</head>
<body>

  <h1>🧾 Rust Console Report Card Generator</h1>

  <p>A simple command-line application built in <strong>Rust</strong> that:</p>
  <ul>
    <li>Accepts student information</li>
    <li>Calculates average marks using a custom function</li>
    <li>Assigns a grade based on performance</li>
    <li>Generates a professional <strong>PDF report card</strong></li>
  </ul>

  <hr>

  <h2>🚀 Features</h2>
  <ul>
    <li>📥 Console-based input for name, total marks, and subjects</li>
    <li>🧠 Custom average calculator</li>
    <li>🏷️ Grade assignment logic:
      <ul>
        <li><strong>A</strong>: 90+</li>
        <li><strong>B</strong>: 75–89</li>
        <li><strong>C</strong>: 60–74</li>
        <li><strong>D</strong>: Below 60</li>
      </ul>
    </li>
    <li>📄 Generates a PDF using <a href="https://crates.io/crates/printpdf" target="_blank"><code>printpdf</code></a></li>
  </ul>

</body>
</html>

---

<h3>2. Install Dependencies</h3>
<p>Ensure <a href="https://www.rust-lang.org/tools/install" target="_blank">Rust</a> is installed.</p>

<pre><code class="language-bash">
cargo build
</code></pre>

<h3>3. Run the App</h3>

<pre><code class="language-bash">
cargo run
</code></pre>

<hr>

<h3>🧾 Dependencies</h3>
<ul>
  <li><code>printpdf</code> – For generating PDF files</li>
  <li>Standard Rust Libraries</li>
</ul>


