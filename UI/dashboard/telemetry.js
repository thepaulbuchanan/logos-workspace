let globalTraceData = [];
let currentPillar = 'epistemology';
let typingTimer;
const doneTypingInterval = 400;

const editor = document.getElementById('executiveEditor');
const syncStatus = document.getElementById('syncStatus');
const activeProjDisplay = document.getElementById('activeProjectDisplay');

if (editor) {
    editor.addEventListener('input', () => {
        clearTimeout(typingTimer);
        syncStatus.innerText = "Analyzing text stream...";
        typingTimer = setTimeout(pushTextStreamDownstream, doneTypingInterval);
    });
}

// 📂 OPERATION 1: INITIALIZE DYNAMIC SANDBOX WORKSPACE
async function createNewProjectWorkspace() {
    const projName = document.getElementById('projectNameInput').value.trim();
    if (!projName) return;

    const response = await fetch('/api/create-project', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: projName })
    });
    const data = await response.json();
    if (data.status === 'success') {
        activeProjDisplay.innerText = data.project;
        editor.value = `% Initialised Invariant Canvas Workspace for Project: ${data.project}\n\n`;
        document.getElementById('fileNameDisplay').innerText = "";
        pushTextStreamDownstream();
    }
}

// 📥 OPERATION 2: MULTI-PART REFERENCE FILE IMPORT
async function executeFileImportPayload() {
    const fileInput = document.getElementById('fileImportInput');
    if (fileInput.files.length === 0) return;

    const file = fileInput.files[0];
    const formData = new FormData();
    formData.append('file', file);
    formData.append('filename', file.name);

    syncStatus.innerText = "Uploading reference file...";
    
    const response = await fetch('/api/upload-file', {
        method: 'POST',
        body: formData
    });
    const data = await response.json();
    if (data.status === 'success') {
        document.getElementById('fileNameDisplay').innerText = `Linked: ${data.filename}`;
        pushTextStreamDownstream();
    }
}

async function pushTextStreamDownstream() {
    try {
        const textPayload = editor ? editor.value : '';
        const response = await fetch('/api/evaluate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ text: textPayload })
        });
        const data = await response.json();
        
        document.getElementById('count-epistemology').innerText = data.epistemology_total;
        document.getElementById('count-ontology').innerText = data.ontology_total;
        document.getElementById('count-phenomenology').innerText = data.phenomenology_total;
        
        syncStatus.innerText = "Workspace Synchronized";
        syncStatus.style.color = "var(--accent-clean)";
        globalTraceData = data.trace;
        renderActivePillar();
    } catch (e) {
        console.error(e);
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
            <div class="card-body"><strong>[${block.environment}] Invariant Context:</strong> "${block.clause}"</div>
        `;
        container.appendChild(card);
    });
}

pushTextStreamDownstream();
