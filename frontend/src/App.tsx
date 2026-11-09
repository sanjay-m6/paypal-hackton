import { useState } from 'react'
import {
  ShieldAlert,
  ShieldCheck,
  Cpu,
  UserCheck,
  Activity,
  Layers,
  FileCode,
  CheckCircle2,
  ChevronRight,
} from 'lucide-react'


// Agent Catalogue Data
const AGENTS = [
  { id: '00', name: 'Master Orchestrator', scope: 'Task planning & agent coordination', owner: 'Kaviya', stage: 'Core' },
  { id: '01', name: 'Transaction Pattern Analyst', scope: 'Anomaly & velocity detection', owner: 'Kaviya', stage: 'Risk' },
  { id: '02', name: 'Identity Verification', scope: 'KYC & signal consistency', owner: 'Kaviya', stage: 'Risk' },
  { id: '03', name: 'Merchant Risk Intelligence', scope: 'Chargeback & operational flags', owner: 'Kaviya', stage: 'Risk' },
  { id: '04', name: 'Device & Session Intelligence', scope: 'Fingerprint & bot detection', owner: 'Kaviya', stage: 'Risk' },
  { id: '05', name: 'Behavioral Intelligence', scope: 'Keystroke & event sequence drift', owner: 'Kaviya', stage: 'Risk' },
  { id: '06', name: 'Payment Intelligence', scope: 'Declines, retries & gateway codes', owner: 'Sanjay', stage: 'Payment' },
  { id: '07', name: 'Checkout Optimization', scope: 'Friction reduction signals', owner: 'Kaviya', stage: 'Commerce' },
  { id: '08', name: 'Customer Intelligence', scope: 'Lifecycle & trust metrics', owner: 'Kaviya', stage: 'Commerce' },
  { id: '09', name: 'Recommendation Engine', scope: 'Explainable payment ranking', owner: 'Kaviya', stage: 'Commerce' },
  { id: '10', name: 'Refund & Return Intelligence', scope: 'Return abuse & policy validation', owner: 'Sanjay', stage: 'Financial' },
  { id: '11', name: 'Dispute Investigation', scope: 'Evidence timeline compilation', owner: 'Sanjay', stage: 'Financial' },
  { id: '12', name: 'Business Operations Intelligence', scope: 'SLA alerts & bottleneck detection', owner: 'Sanjay', stage: 'Financial' },
]

export default function App() {
  const [selectedAgent, setSelectedAgent] = useState(AGENTS[0])
  const [selectedTab, setSelectedTab] = useState<'overview' | 'agents' | 'policy' | 'roadmap'>('overview')

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-indigo-500 selection:text-white">
      {/* Top Navbar */}
      <header className="border-b border-slate-800 bg-slate-900/80 backdrop-blur sticky top-0 z-50 px-6 py-3.5 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="h-9 w-9 rounded-lg bg-gradient-to-tr from-indigo-600 to-cyan-500 flex items-center justify-center shadow-lg shadow-indigo-500/20">
            <ShieldCheck className="h-5 w-5 text-white" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-base font-bold tracking-tight text-white">Payment Security Platform</h1>
              <span className="text-[10px] uppercase font-semibold px-2 py-0.5 rounded-full bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
                Phase 1 Active
              </span>
            </div>
            <p className="text-xs text-slate-400">Multi-Agent Commerce Intelligence · Rust Core Engine</p>
          </div>
        </div>

        {/* Team Badges */}
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 bg-slate-800/80 border border-slate-700/60 rounded-lg px-3 py-1.5 text-xs">
            <span className="text-slate-400">Team:</span>
            <span className="flex items-center gap-1 font-medium text-cyan-400">
              <span className="h-1.5 w-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
              Sanjay (Lead & Backend)
            </span>
            <span className="text-slate-600">·</span>
            <span className="flex items-center gap-1 font-medium text-purple-400">
              <span className="h-1.5 w-1.5 rounded-full bg-purple-400"></span>
              Kaviya (Full-Stack & AI)
            </span>
          </div>

          <div className="flex items-center gap-2 text-xs text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-2.5 py-1.5 rounded-lg">
            <Activity className="h-3.5 w-3.5 animate-pulse" />
            <span>Rust Backend: Healthy</span>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-7xl w-full mx-auto p-6 space-y-6">
        {/* Phase 1 Completion Hero Banner */}
        <div className="relative overflow-hidden rounded-2xl border border-slate-800 bg-gradient-to-r from-slate-900 via-indigo-950/30 to-slate-900 p-6 shadow-xl">
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-6">
            <div className="space-y-2 max-w-2xl">
              <div className="inline-flex items-center gap-1.5 text-xs font-semibold text-emerald-400 bg-emerald-500/10 px-2.5 py-1 rounded-md border border-emerald-500/20">
                <CheckCircle2 className="h-3.5 w-3.5" />
                Phase 1: Product Requirements & Problem Definition
              </div>
              <h2 className="text-2xl font-bold tracking-tight text-white">
                Multi-Agent Payment Security Platform Foundation
              </h2>
              <p className="text-sm text-slate-300 leading-relaxed">
                Core domain models, error classifications, 13 specialist agent contracts, and the deterministic Rust policy engine have been successfully scaffolded and verified.
              </p>
            </div>

            <div className="flex items-center gap-3">
              <div className="bg-slate-800/90 border border-slate-700 rounded-xl p-4 text-center min-w-[120px]">
                <div className="text-2xl font-black text-indigo-400 font-mono">13</div>
                <div className="text-[11px] uppercase tracking-wider text-slate-400 mt-0.5">Specialist Agents</div>
              </div>
              <div className="bg-slate-800/90 border border-slate-700 rounded-xl p-4 text-center min-w-[120px]">
                <div className="text-2xl font-black text-cyan-400 font-mono">7</div>
                <div className="text-[11px] uppercase tracking-wider text-slate-400 mt-0.5">Rust Crates</div>
              </div>
              <div className="bg-slate-800/90 border border-slate-700 rounded-xl p-4 text-center min-w-[120px]">
                <div className="text-2xl font-black text-emerald-400 font-mono">40</div>
                <div className="text-[11px] uppercase tracking-wider text-slate-400 mt-0.5">Total Phases</div>
              </div>
            </div>
          </div>
        </div>

        {/* Navigation Tabs */}
        <div className="flex items-center gap-2 border-b border-slate-800 pb-2">
          <button
            onClick={() => setSelectedTab('overview')}
            className={`px-3.5 py-1.5 text-sm font-medium rounded-lg transition-all ${
              selectedTab === 'overview'
                ? 'bg-indigo-600 text-white shadow-sm'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
            }`}
          >
            Architecture Overview
          </button>
          <button
            onClick={() => setSelectedTab('agents')}
            className={`px-3.5 py-1.5 text-sm font-medium rounded-lg transition-all ${
              selectedTab === 'agents'
                ? 'bg-indigo-600 text-white shadow-sm'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
            }`}
          >
            Specialist Agents (13)
          </button>
          <button
            onClick={() => setSelectedTab('policy')}
            className={`px-3.5 py-1.5 text-sm font-medium rounded-lg transition-all ${
              selectedTab === 'policy'
                ? 'bg-indigo-600 text-white shadow-sm'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
            }`}
          >
            Policy Engine & Safety
          </button>
          <button
            onClick={() => setSelectedTab('roadmap')}
            className={`px-3.5 py-1.5 text-sm font-medium rounded-lg transition-all ${
              selectedTab === 'roadmap'
                ? 'bg-indigo-600 text-white shadow-sm'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900'
            }`}
          >
            40-Phase Roadmap
          </button>
        </div>

        {/* Tab 1: Architecture Overview */}
        {selectedTab === 'overview' && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {/* Box 1: Core Flow */}
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-3">
              <div className="flex items-center gap-2 text-indigo-400 font-semibold text-sm">
                <Layers className="h-4 w-4" />
                Deterministic Trust Boundary
              </div>
              <p className="text-xs text-slate-300 leading-relaxed">
                The frontend and AI agents are strictly <strong>advisory</strong>. No model response or browser action can trigger financial moves. Only the pure Rust Policy Engine issues final decisions.
              </p>
              <div className="space-y-2 pt-2">
                <div className="text-xs flex items-center justify-between p-2 rounded bg-slate-800/80 border border-slate-700/60">
                  <span className="text-slate-400">1. Client Request</span>
                  <span className="font-mono text-cyan-400">Axum API</span>
                </div>
                <div className="text-xs flex items-center justify-between p-2 rounded bg-slate-800/80 border border-slate-700/60">
                  <span className="text-slate-400">2. Agent Orchestration</span>
                  <span className="font-mono text-purple-400">13 AI Agents</span>
                </div>
                <div className="text-xs flex items-center justify-between p-2 rounded bg-slate-800/80 border border-slate-700/60">
                  <span className="text-slate-400">3. Policy Authority</span>
                  <span className="font-mono text-emerald-400">Rust Engine</span>
                </div>
                <div className="text-xs flex items-center justify-between p-2 rounded bg-slate-800/80 border border-slate-700/60">
                  <span className="text-slate-400">4. Payment Execution</span>
                  <span className="font-mono text-amber-400">PayPal Sandbox</span>
                </div>
              </div>
            </div>

            {/* Box 2: Team Ownership Matrix */}
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-3">
              <div className="flex items-center gap-2 text-cyan-400 font-semibold text-sm">
                <UserCheck className="h-4 w-4" />
                2-Person Team Ownership
              </div>
              <div className="space-y-3 text-xs">
                <div className="p-3 rounded-lg bg-slate-800/60 border border-slate-700">
                  <div className="flex items-center justify-between font-bold text-cyan-300 mb-1">
                    <span>Sanjay</span>
                    <span className="text-[10px] px-1.5 py-0.5 rounded bg-cyan-950 text-cyan-400 border border-cyan-800">Team Lead</span>
                  </div>
                  <p className="text-slate-400 leading-relaxed">
                    Rust backend, Axum API, database (SQLx), Policy Engine, PayPal gateway adapter, idempotency, security audits.
                  </p>
                </div>
                <div className="p-3 rounded-lg bg-slate-800/60 border border-slate-700">
                  <div className="flex items-center justify-between font-bold text-purple-300 mb-1">
                    <span>Kaviya</span>
                    <span className="text-[10px] px-1.5 py-0.5 rounded bg-purple-950 text-purple-400 border border-purple-800">Full-Stack & AI</span>
                  </div>
                  <p className="text-slate-400 leading-relaxed">
                    React + TypeScript UI, Agent Orchestrator, Specialist Agent prompts & schemas, evidence aggregation, operator UX.
                  </p>
                </div>
              </div>
            </div>

            {/* Box 3: Rust Workspace Structure */}
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 space-y-3">
              <div className="flex items-center gap-2 text-emerald-400 font-semibold text-sm">
                <FileCode className="h-4 w-4" />
                7 Backend Crates Initialized
              </div>
              <div className="space-y-1.5 font-mono text-xs text-slate-300">
                <div className="flex items-center justify-between py-1 border-b border-slate-800">
                  <span className="text-indigo-400">payment-core</span>
                  <span className="text-slate-500 text-[10px]">Domain Types</span>
                </div>
                <div className="flex items-center justify-between py-1 border-b border-slate-800">
                  <span className="text-indigo-400">payment-policy</span>
                  <span className="text-slate-500 text-[10px]">Rules Engine</span>
                </div>
                <div className="flex items-center justify-between py-1 border-b border-slate-800">
                  <span className="text-indigo-400">payment-agents</span>
                  <span className="text-slate-500 text-[10px]">Registry & Runner</span>
                </div>
                <div className="flex items-center justify-between py-1 border-b border-slate-800">
                  <span className="text-indigo-400">payment-paypal</span>
                  <span className="text-slate-500 text-[10px]">Gateway Adapter</span>
                </div>
                <div className="flex items-center justify-between py-1 border-b border-slate-800">
                  <span className="text-indigo-400">payment-db</span>
                  <span className="text-slate-500 text-[10px]">SQLx & Pool</span>
                </div>
                <div className="flex items-center justify-between py-1 border-b border-slate-800">
                  <span className="text-indigo-400">payment-audit</span>
                  <span className="text-slate-500 text-[10px]">Telemetry</span>
                </div>
                <div className="flex items-center justify-between py-1">
                  <span className="text-indigo-400">payment-api</span>
                  <span className="text-slate-500 text-[10px]">Axum Server</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Tab 2: Specialist Agents */}
        {selectedTab === 'agents' && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="md:col-span-1 border border-slate-800 bg-slate-900/60 rounded-xl overflow-hidden divide-y divide-slate-800/80 max-h-[540px] overflow-y-auto">
              {AGENTS.map((agent) => (
                <button
                  key={agent.id}
                  onClick={() => setSelectedAgent(agent)}
                  className={`w-full p-3 text-left flex items-center justify-between transition-colors ${
                    selectedAgent.id === agent.id ? 'bg-indigo-600/20 text-white' : 'hover:bg-slate-800/40 text-slate-300'
                  }`}
                >
                  <div className="flex items-center gap-2.5">
                    <span className="font-mono text-xs font-bold px-1.5 py-0.5 rounded bg-slate-800 text-indigo-400">
                      {agent.id}
                    </span>
                    <div>
                      <div className="text-xs font-semibold">{agent.name}</div>
                      <div className="text-[10px] text-slate-400">{agent.scope}</div>
                    </div>
                  </div>
                  <ChevronRight className="h-4 w-4 text-slate-500" />
                </button>
              ))}
            </div>

            {/* Agent Detail Card */}
            <div className="md:col-span-2 border border-slate-800 bg-slate-900/80 rounded-xl p-6 space-y-4">
              <div className="flex items-center justify-between border-b border-slate-800 pb-4">
                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-mono text-xs px-2 py-0.5 rounded bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
                      Agent ID: {selectedAgent.id}
                    </span>
                    <span className="text-xs px-2 py-0.5 rounded bg-slate-800 text-slate-300">
                      Domain: {selectedAgent.stage}
                    </span>
                  </div>
                  <h3 className="text-lg font-bold text-white mt-1">{selectedAgent.name}</h3>
                </div>

                <div className="text-right">
                  <span className="text-[11px] text-slate-400">Lead Owner</span>
                  <div className={`text-xs font-semibold ${selectedAgent.owner === 'Sanjay' ? 'text-cyan-400' : 'text-purple-400'}`}>
                    {selectedAgent.owner}
                  </div>
                </div>
              </div>

              <div className="space-y-3 text-xs text-slate-300">
                <div>
                  <h4 className="font-semibold text-slate-200 mb-1">Operational Scope:</h4>
                  <p className="text-slate-400">{selectedAgent.scope}</p>
                </div>

                <div className="p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/20 text-amber-300 space-y-1">
                  <div className="flex items-center gap-1.5 font-bold">
                    <ShieldAlert className="h-4 w-4" />
                    Strict Operational Limitation
                  </div>
                  <p className="text-[11px] text-amber-200/90 leading-relaxed">
                    This agent operates under read-only sandboxed permissions. It returns structured hypotheses and evidence refs. It cannot alter payment records, execute captures, or approve transactions directly.
                  </p>
                </div>

                <div className="grid grid-cols-2 gap-3 pt-2">
                  <div className="p-3 rounded-lg bg-slate-800/60 border border-slate-700/60">
                    <span className="text-[10px] text-slate-400 uppercase font-semibold">Response Contract</span>
                    <div className="font-mono text-xs text-indigo-300 mt-1">payment_core::AgentResponse</div>
                  </div>
                  <div className="p-3 rounded-lg bg-slate-800/60 border border-slate-700/60">
                    <span className="text-[10px] text-slate-400 uppercase font-semibold">Execution Timeout</span>
                    <div className="font-mono text-xs text-emerald-400 mt-1">500ms max (enforced)</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Tab 3: Policy Engine & Safety */}
        {selectedTab === 'policy' && (
          <div className="border border-slate-800 bg-slate-900/60 rounded-xl p-6 space-y-6">
            <div className="space-y-1">
              <h3 className="text-lg font-bold text-white flex items-center gap-2">
                <Cpu className="h-5 w-5 text-indigo-400" />
                Deterministic Policy Outcomes
              </h3>
              <p className="text-xs text-slate-400">
                The engine processes aggregated findings and outputs exactly one of four auditable decision states:
              </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div className="p-4 rounded-xl border border-emerald-500/30 bg-emerald-950/20 space-y-2">
                <span className="font-mono text-xs font-bold text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
                  ALLOW
                </span>
                <p className="text-xs text-slate-300">Transaction cleared. PayPal adapter executes authorization or capture with idempotency keys.</p>
              </div>

              <div className="p-4 rounded-xl border border-amber-500/30 bg-amber-950/20 space-y-2">
                <span className="font-mono text-xs font-bold text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded border border-amber-500/20">
                  STEP_UP
                </span>
                <p className="text-xs text-slate-300">Identity ambiguity detected. Prompts 3D Secure / MFA verification before allowing payment.</p>
              </div>

              <div className="p-4 rounded-xl border border-blue-500/30 bg-blue-950/20 space-y-2">
                <span className="font-mono text-xs font-bold text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded border border-blue-500/20">
                  MANUAL_REVIEW
                </span>
                <p className="text-xs text-slate-300">Dispatched to fraud analyst queue with structured evidence timeline and reasoning codes.</p>
              </div>

              <div className="p-4 rounded-xl border border-rose-500/30 bg-rose-950/20 space-y-2">
                <span className="font-mono text-xs font-bold text-rose-400 bg-rose-500/10 px-2 py-0.5 rounded border border-rose-500/20">
                  DENY
                </span>
                <p className="text-xs text-slate-300">Hard reject. High confidence fraud, compromised device, or sanctioned entity detected.</p>
              </div>
            </div>
          </div>
        )}

        {/* Tab 4: 40-Phase Roadmap */}
        {selectedTab === 'roadmap' && (
          <div className="border border-slate-800 bg-slate-900/60 rounded-xl p-6 space-y-4">
            <h3 className="text-base font-bold text-white">40-Phase Development Stages (Sanjay & Kaviya)</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
              <div className="p-4 rounded-lg bg-slate-800/60 border border-slate-700/80 space-y-1">
                <div className="flex items-center justify-between font-bold text-emerald-400">
                  <span>Stage A: Phases 1–5 (Foundation)</span>
                  <span className="text-[10px] bg-emerald-500/20 px-2 py-0.5 rounded">Phase 1 Done</span>
                </div>
                <p className="text-slate-400">Requirements, Architecture, Scaffolding, Data Model & Tenancy. Owner: Sanjay (Lead).</p>
              </div>

              <div className="p-4 rounded-lg bg-slate-800/60 border border-slate-700/80 space-y-1">
                <div className="flex items-center justify-between font-bold text-indigo-400">
                  <span>Stage B: Phases 6–10 (Rust Core & Policy)</span>
                  <span className="text-[10px] bg-indigo-500/20 px-2 py-0.5 rounded">Next Up</span>
                </div>
                <p className="text-slate-400">Axum API, Event Bus, Agent Registry, Policy Engine, Audit Telemetry. Owner: Sanjay.</p>
              </div>

              <div className="p-4 rounded-lg bg-slate-800/60 border border-slate-700/80 space-y-1">
                <div className="flex items-center justify-between font-bold text-purple-400">
                  <span>Stage C: Phases 11–17 (Agent Intelligence)</span>
                  <span className="text-[10px] bg-purple-500/20 px-2 py-0.5 rounded">Planned</span>
                </div>
                <p className="text-slate-400">Master Orchestrator, Risk Specialist Agents (01-05), Collaborative Aggregator. Owner: Kaviya.</p>
              </div>

              <div className="p-4 rounded-lg bg-slate-800/60 border border-slate-700/80 space-y-1">
                <div className="flex items-center justify-between font-bold text-cyan-400">
                  <span>Stage D: Phases 18–24 (Commerce & Financial)</span>
                  <span className="text-[10px] bg-cyan-500/20 px-2 py-0.5 rounded">Planned</span>
                </div>
                <p className="text-slate-400">Payment, Checkout, Recs, Refunds, Disputes, Ops. Split: Sanjay & Kaviya.</p>
              </div>
            </div>
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="border-t border-slate-800 py-4 px-6 text-center text-xs text-slate-500 mt-auto">
        Multi-Agent AI Payment Security Platform · Hackathon Release · Team Lead: Sanjay · Full-Stack & AI: Kaviya
      </footer>
    </div>
  )
}
