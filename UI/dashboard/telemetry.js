let globalTraceData = [];
let currentPillar = 'epistemology';

async function fetchTelemetryData() {
    try {
        const response = await fetch('/api/lemmas');
        const data = await response.json();
        
        document.getElementById('totalLemmas').innerText = data.total_library_lemmas;
        document.getElementById('totalChunks').innerText = data.total_chunks;
        document.getElementById('cleanChunks').innerText = `${data.clean_count} [✓]`;
        document.getElementById('quarantinedChunks').innerText = `${data.quarantine_count} [✗]`;
        document.getElementById('objectivityPct').innerText = `${data.objectivity_score}%`;
        
        document.getElementById('count-epistemology').innerText = data.epistemology_total;
        document.getElementById('count-ontology').innerText = data.ontology_total;
        document.getElementById('count-phenomenology').innerText = data.phenomenology_total;
        
        const gauge = document.getElementById('metricGauge');
        gauge.style.background = `conic-gradient(var(--accent-clean) 0% ${data.objectivity_score}%, var(--border-color) ${data.objectivity_score}% 100%)`;

        globalTraceData = data.trace;
        renderActivePillar();
    } catch (e) {
        console.error("Telemetry channel ingestion fault:", e);
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
        container.innerHTML = '<p class="text-muted">No fault nodes registered inside this classification pillar.</p>';
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

// Initialise execution polling loop on initialization
fetchTelemetryData();
