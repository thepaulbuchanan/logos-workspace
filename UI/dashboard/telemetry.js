let globalTraceData = [];
let currentPillar = 'epistemology';
let typingTimer;
const doneTypingInterval = 400; // 400ms Debounce Threshold to prevent compiler lag

const editor = document.getElementById('executiveEditor');
const syncStatus = document.getElementById('syncStatus');

// Monitor keystroke inputs and handle live timing resets
if (editor) {
    editor.addEventListener('input', () => {
        clearTimeout(typingTimer);
        syncStatus.innerText = "Analyzing text stream...";
        syncStatus.style.color = "var(--text-muted)";
        typingTimer = setTimeout(pushTextStreamDownstream, doneTypingInterval);
    });
}

async function pushTextStreamDownstream() {
    try {
        const textPayload = editor.value;
        
        // FIX: Enforced strict native JSON serialization casing to resolve browser console crashes
        const response = await fetch('/api/evaluate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ text: textPayload })
        });
        
        const data = await response.json();
        
        // Update metric row element trackers
        document.getElementById('count-epistemology').innerText = data.epistemology_total;
        document.getElementById('count-ontology').innerText = data.ontology_total;
        document.getElementById('count-phenomenology').innerText = data.phenomenology_total;
        
        syncStatus.innerText = "Epistemic Firewall Synced";
        syncStatus.style.color = "var(--accent-clean)";

        globalTraceData = data.trace;
        renderActivePillar();
    } catch (e) {
        console.error("In-memory streaming channel failure:", e);
        syncStatus.innerText = "Connection Broken";
        syncStatus.style.color = "var(--accent-quarantine)";
    }
}

function switchPillar(pillarName) {
    currentPillar = pillarName;
    document.querySelectorAll('.tab-bar').forEach(btn => btn.classList.remove('active'));
    document.getElementById(`tab-${pillarName}`).classList.add('active');
    renderActivePillar();
}

function renderActivePillar() {
    const container = document.getElementById('traceContainer');
    container.innerHTML = '';
    
    const filtered = globalTraceData.filter(b => b.pillar === currentPillar);
    if (filtered.length === 0) {
        container.innerHTML = '<p class="text-muted">No logical flaws isolated inside this classification node matrix.</p>';
        return;
    }

    filtered.forEach((block) => {
        const card = document.createElement('div');
        card.className = 'block-card';
        let badgeClass = block.status === 'quarantine' ? 'badge-quarantine' : (block.status === 'math' ? 'badge-math' : 'badge-clean');
        let badgeLabel = block.status === 'quarantine' ? `🔴 ${block.lemma_id}` : (block.status === 'math' ? '🔵 LEAN4 PROVED' : '🟢 VERIFIED CLEAN');

        card.innerHTML = `
            <div class="card-header">
                <span>Chunk ${block.index} <span style="color: var(--text-muted); font-size: 0.85rem; margin-left: 0.5rem;">(Source Line: ${block.line})</span></span>
                <span class="card-badge ${badgeClass}">${badgeLabel}</span>
            </div>
            <div class="card-body">
                <strong>[${block.environment}] Invariant Context:</strong> "${block.clause}"
            </div>
        `;
        container.appendChild(card);
    });
}

// Initialise compilation loop on boot sequence
pushTextStreamDownstream();
