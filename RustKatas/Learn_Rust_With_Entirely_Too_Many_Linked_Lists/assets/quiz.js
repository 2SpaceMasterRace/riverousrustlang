/* ===========================================================================
   Shared interactive widgets for lessons. Vanilla JS, no dependencies.
   Reused across every lesson — do not inline copies into individual lessons.

   Widgets (declared with data-attributes, wired up on DOMContentLoaded):

   1. Multiple choice
      <div class="quiz" data-answer="b">
        <p class="q">Question text</p>
        <button data-opt="a">option one</button>
        <button data-opt="b">option two</button>
        <p class="why" hidden>Explanation shown after answering.</p>
      </div>

   2. Flip card (retrieval practice — recall, then check)
      <div class="flip">
        <div class="front">Prompt: write the type of Link</div>
        <div class="back" hidden>enum Link { Empty, More(Box&lt;Node&gt;) }</div>
      </div>

   3. Type-to-check (blind reproduction — type it, compare to target)
      <div class="type-check" data-target="enum Link { Empty, More(Box<Node>) }">
        <p class="q">From memory, define the Link enum:</p>
        <textarea rows="3"></textarea>
        <button class="check">Check</button>
        <p class="verdict" hidden></p>
      </div>
   =========================================================================== */

document.addEventListener("DOMContentLoaded", () => {

  /* ---- Multiple choice ------------------------------------------------- */
  document.querySelectorAll(".quiz").forEach((quiz) => {
    const answer = quiz.dataset.answer;
    const why = quiz.querySelector(".why");
    quiz.querySelectorAll("button[data-opt]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const correct = btn.dataset.opt === answer;
        quiz.querySelectorAll("button[data-opt]").forEach((b) => {
          b.disabled = true;
          if (b.dataset.opt === answer) b.classList.add("right");
          else if (b === btn) b.classList.add("wrong");
        });
        if (why) why.hidden = false;
        quiz.classList.add(correct ? "answered-right" : "answered-wrong");
      });
    });
  });

  /* ---- Flip card ------------------------------------------------------- */
  document.querySelectorAll(".flip").forEach((card) => {
    const back = card.querySelector(".back");
    const hint = document.createElement("span");
    hint.className = "flip-hint";
    hint.textContent = "click to reveal";
    card.appendChild(hint);
    card.addEventListener("click", () => {
      const show = back.hidden;
      back.hidden = !show;
      hint.textContent = show ? "click to hide" : "click to reveal";
    });
  });

  /* ---- Type-to-check (fuzzy: ignore whitespace + trailing punctuation) - */
  const norm = (s) =>
    s.replace(/\s+/g, " ").replace(/\s*([{}(),;<>])\s*/g, "$1").trim().toLowerCase();

  document.querySelectorAll(".type-check").forEach((tc) => {
    const target = tc.dataset.target;
    const ta = tc.querySelector("textarea");
    const verdict = tc.querySelector(".verdict");
    tc.querySelector(".check").addEventListener("click", () => {
      verdict.hidden = false;
      if (norm(ta.value) === norm(target)) {
        verdict.textContent = "✓ Exact match. You've got it.";
        verdict.className = "verdict ok";
      } else {
        verdict.innerHTML =
          "✗ Not quite. Target:<br><code>" +
          target.replace(/</g, "&lt;").replace(/>/g, "&gt;") +
          "</code>";
        verdict.className = "verdict no";
      }
    });
  });
});
