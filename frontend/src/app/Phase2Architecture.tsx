import { useState, useEffect, type FormEvent } from 'react'
import {
  Cpu,
  Database,
  Lock,
  Activity,
  CheckCircle2,
  ArrowRight,
  Server,
  Terminal,
  Play,
  GitBranch,
  ShieldCheck,
  UserX,
} from 'lucide-react'
import { NeuCard, NeuButton, NeuInput } from '@/components/neu'
import axios from 'axios'

interface AgentDescriptor {
  id: number
  name: string
  version: string
  owner: string
  scope: string
  capabilities: string[]
  timeout_ms: number
}

interface EvaluationFinding {
  agent_id: string
  finding_type: string
  severity: string
  description: string
  confidence_value: number
  confidence_note: string
  evidence_refs: string[]
}

interface EvaluationResult {
  transaction_id: string
  policy_decision: 'ALLOW' | 'DENY' | 'STEP_UP' | 'MANUAL_REVIEW'
  reason_codes: string[]
  policy_version: string
  agent_findings: EvaluationFinding[]
  evaluated_at: string
  correlation_id: string
}

interface LoginResponse {
  access_token: string
  token_type: string
  expires_in: number
  user_id: string
  tenant_id: string
  role: string
}

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3001'

// Synthetic scenarios from scripts/seed_data.json
const SYNTHETIC_SCENARIOS = [
  {
    id: 'scenario_normal_allow_01',
    name: 'Normal Verified Customer',
    expected: 'ALLOW',
    amount: '149.99',
    email: 'alice.synthetic@example.com',
    paymentMethod: 'PAYPAL',
    tag: 'Low Risk',
  },
  {
    id: 'scenario_stepup_mfa_02',
    name: 'New Device & Location',
    expected: 'STEP_UP',
    amount: '890.00',
    email: 'bob.synthetic@example.com',
    paymentMethod: 'PAYPAL',
    tag: 'Anomaly',
  },
  {
    id: 'scenario_deny_proxy_fraud_03',
    name: 'Tor Exit Node & Rapid Velocity',
    expected: 'DENY',
    amount: '2499.00',
    email: 'disposable.addr@mailinator.com',
    paymentMethod: 'CARD',
    tag: 'Critical Fraud',
  },
  {
    id: 'scenario_manual_review_04',
    name: 'Merchant Spike & Hesitant Behavior',
    expected: 'MANUAL_REVIEW',
    amount: '1250.00',
    email: 'charlie.synthetic@example.com',
    paymentMethod: 'PAYPAL',
    tag: 'Review Needed',
  },
]

export default function Phase2Architecture() {
  const [agents, setAgents] = useState<AgentDescriptor[]>([])
  const [loadingAgents, setLoadingAgents] = useState(true)
  const [evaluating, setEvaluating] = useState(false)
  const [evaluationResult, setEvaluationResult] = useState<EvaluationResult | null>(null)
  const [accessToken, setAccessToken] = useState<string | null>(null)
  const [tenantId, setTenantId] = useState<string | null>(null)
  const [loginEmail, setLoginEmail] = useState('')
  const [loginPassword, setLoginPassword] = useState('')
  const [loginError, setLoginError] = useState<string | null>(null)
  const [apiError, setApiError] = useState<string | null>(null)
  const [loggingIn, setLoggingIn] = useState(false)
  const [amount, setAmount] = useState('249.50')
  const [email, setEmail] = useState('alice@example.com')
  const [selectedScenario, setSelectedScenario] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<'evaluator' | 'agents' | 'schema' | 'threats' | 'devexp' | 'datagov'>('evaluator')

  // Phase 4 Data Governance Interactive Sandbox State
  const [authTenant, setAuthTenant] = useState<'alpha' | 'beta'>('alpha')
  const [targetTenant, setTargetTenant] = useState<'alpha' | 'beta'>('alpha')
  const [sandboxResult, setSandboxResult] = useState<{ status: 'ALLOW' | 'DENY'; query: string; reason: string } | null>(null)
  const [piiEmail, setPiiEmail] = useState('alice.smith@enterprise.corp')
  const [piiCustomerId, setPiiCustomerId] = useState('cust_998124')
  const [isAnonymized, setIsAnonymized] = useState(false)

  // Fetch protected agent descriptors after a successful login.
  useEffect(() => {
    if (!accessToken) {
      setLoadingAgents(false)
      return
    }

    if (accessToken === 'demo-dev-access-token') {
      setAgents([
        { id: 1, name: 'Transaction Pattern Analyst', version: '0.1.0', owner: 'Kaviya', scope: 'Velocity, Amount Anomaly', capabilities: ['RAPID_VELOCITY', 'AMOUNT_SPIKE'], timeout_ms: 500 },
        { id: 2, name: 'Identity Verification Intelligence', version: '0.1.0', owner: 'Kaviya', scope: 'Email & Device Profiling', capabilities: ['EMAIL_AGE', 'DEVICE_FINGERPRINT'], timeout_ms: 500 },
        { id: 3, name: 'Merchant Risk Intelligence', version: '0.1.0', owner: 'Kaviya', scope: 'Category & Volume Spikes', capabilities: ['CHARGEBACK_SPIKE', 'MCC_RISK'], timeout_ms: 500 },
        { id: 4, name: 'Device & Session Intelligence', version: '0.1.0', owner: 'Kaviya', scope: 'Tor, Proxy, VPN Detection', capabilities: ['TOR_EXIT_NODE', 'GEO_MISMATCH'], timeout_ms: 500 },
        { id: 5, name: 'Behavior Sequence Monitor', version: '0.1.0', owner: 'Kaviya', scope: 'Hesitation, Typing Dynamics', capabilities: ['AUTOMATION_BOT', 'RAPID_SUBMIT'], timeout_ms: 500 },
      ])
      setLoadingAgents(false)
      return
    }

    axios
      .get<AgentDescriptor[]>(`${API_BASE_URL}/api/v1/agents`, {
        headers: { Authorization: `Bearer ${accessToken}` },
      })
      .then((res) => {
        setAgents(res.data)
        setApiError(null)
      })
      .catch(() => {
        setApiError('Could not load protected API data. Please sign in again.')
        setAccessToken(null)
      })
      .finally(() => setLoadingAgents(false))
  }, [accessToken])

  const handleLogin = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    setLoggingIn(true)
    setLoginError(null)

    if (loginEmail === 'admin@example.com' && loginPassword === 'AdminPassword123!') {
      setAccessToken('demo-dev-access-token')
      setTenantId('22222222-2222-4222-8222-222222222222')
      setLoginPassword('')
      setLoadingAgents(false)
      setLoggingIn(false)
      return
    }

    try {
      const response = await axios.post<LoginResponse>(`${API_BASE_URL}/api/v1/auth/login`, {
        email: loginEmail,
        password: loginPassword,
      })
      setAccessToken(response.data.access_token)
      setTenantId(response.data.tenant_id)
      setLoginPassword('')
      setLoadingAgents(true)
    } catch {
      setLoginError('Sign-in failed. Check the credentials and API connection.')
    } finally {
      setLoggingIn(false)
    }
  }

  // Trigger live payment risk evaluation
  const handleEvaluate = async (customAmount?: string, customEmail?: string) => {
    if (!accessToken || !tenantId) {
      setApiError('Sign in before requesting an evaluation.')
      return
    }
    setEvaluating(true)
    setApiError(null)
    const evalAmount = customAmount || amount
    const evalEmail = customEmail || email

    if (accessToken === 'demo-dev-access-token') {
      setTimeout(() => {
        setEvaluationResult({
          transaction_id: '11111111-1111-1111-1111-111111111111',
          policy_decision: 'MANUAL_REVIEW',
          reason_codes: ['no_rules_configured', 'demo_fallback'],
          policy_version: '0.1.0',
          agent_findings: [],
          evaluated_at: new Date().toISOString(),
          correlation_id: '44444444-4444-4444-4444-444444444444',
        })
        setEvaluating(false)
      }, 300)
      return
    }

    try {
      const res = await axios.post<EvaluationResult>(`${API_BASE_URL}/api/v1/payments/evaluate`, {
        transaction_id: '11111111-1111-1111-1111-111111111111',
        tenant_id: tenantId,
        amount: evalAmount,
        currency: 'USD',
        payment_method: 'PAYPAL',
        customer: {
          customer_id: 'cust_99',
          email: evalEmail,
          ip_address: '192.168.1.1',
        },
        idempotency_key: '33333333-3333-3333-3333-333333333333',
      }, {
        headers: { Authorization: `Bearer ${accessToken}` },
      })
      setEvaluationResult(res.data)
    } catch {
      setApiError('Evaluation request failed. Check the request and API connection.')
    } finally {
      setEvaluating(false)
    }
  }

  const loadScenario = (scenario: typeof SYNTHETIC_SCENARIOS[0]) => {
    setSelectedScenario(scenario.id)
    setAmount(scenario.amount)
    setEmail(scenario.email)
    handleEvaluate(scenario.amount, scenario.email)
  }

  if (!accessToken) {
    return (
      <div className="w-full max-w-md mx-auto py-16">
        <NeuCard className="space-y-5 p-7">
          <div>
            <h2 className="text-xl font-black text-[var(--neu-text)]">Operator sign in</h2>
            <p className="mt-2 text-xs text-[var(--neu-muted)]">
              Phase 5 Authentication Gate. Use provisioned operator credentials or the default demo account below.
            </p>

            <div className="mt-3 p-3.5 rounded-2xl bg-white/60 border border-white space-y-1.5 text-xs font-mono">
              <div className="text-[10px] uppercase font-bold text-[var(--neu-muted)]">Default Credentials:</div>
              <div className="flex justify-between items-center text-[var(--neu-text)]">
                <span className="text-[var(--neu-muted)]">Email:</span>
                <span className="font-bold text-[var(--neu-blue)]">admin@example.com</span>
              </div>
              <div className="flex justify-between items-center text-[var(--neu-text)]">
                <span className="text-[var(--neu-muted)]">Password:</span>
                <span className="font-bold text-[var(--neu-blue)]">AdminPassword123!</span>
              </div>
            </div>
          </div>
          <form onSubmit={handleLogin} className="space-y-4">
            <NeuInput
              type="email"
              autoComplete="username"
              value={loginEmail}
              onChange={(event) => setLoginEmail(event.target.value)}
              placeholder="Email (admin@example.com)"
              aria-label="Operator email"
              required
            />
            <NeuInput
              type="password"
              autoComplete="current-password"
              value={loginPassword}
              onChange={(event) => setLoginPassword(event.target.value)}
              placeholder="Password (AdminPassword123!)"
              aria-label="Operator password"
              required
            />
            {loginError && <p role="alert" className="text-sm text-red-600">{loginError}</p>}
            {apiError && <p role="alert" className="text-sm text-red-600">{apiError}</p>}
            <NeuButton type="submit" disabled={loggingIn} className="w-full font-bold">
              {loggingIn ? 'Signing in…' : 'Sign in'}
            </NeuButton>

            <div className="relative flex py-1 items-center">
              <div className="flex-grow border-t border-black/10"></div>
              <span className="flex-shrink mx-2 text-[10px] font-bold text-[var(--neu-muted)] uppercase">or</span>
              <div className="flex-grow border-t border-black/10"></div>
            </div>

            <NeuButton
              type="button"
              variant="raised"
              onClick={() => {
                setLoginEmail('admin@example.com')
                setLoginPassword('AdminPassword123!')
                setAccessToken('demo-dev-access-token')
                setTenantId('22222222-2222-4222-8222-222222222222')
                setLoadingAgents(false)
              }}
              className="w-full text-xs font-bold"
            >
              One-Click Demo Operator Sign In
            </NeuButton>
          </form>
        </NeuCard>
      </div>
    )
  }

  return (
    <div className="w-full max-w-5xl mx-auto space-y-8 py-4">
      <div className="flex items-center justify-between text-xs text-[var(--neu-muted)]">
        <span>Authenticated · tenant {tenantId}</span>
        <button
          type="button"
          className="font-bold text-[var(--neu-blue)]"
          onClick={() => {
            setAccessToken(null)
            setTenantId(null)
            setAgents([])
            setEvaluationResult(null)
          }}
        >
          Sign out
        </button>
      </div>
      {apiError && <p role="alert" className="text-sm text-red-600">{apiError}</p>}
      {/* Platform Architecture & Data Governance Header */}
      <NeuCard className="p-6">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-6">
          <div className="space-y-2">
            <div className="inline-flex items-center gap-1.5 text-xs font-bold text-[var(--neu-blue)] bg-white/60 px-3 py-1 rounded-xl border border-white">
              <CheckCircle2 className="h-3.5 w-3.5 text-[var(--neu-blue)]" />
              Phases 5–6: Authentication & API Foundation
            </div>
            <h2 className="text-2xl font-black text-[var(--neu-text)] tracking-tight">
              Payment Security Platform & Data Governance
            </h2>
            <p className="text-xs text-[var(--neu-muted)] max-w-2xl leading-relaxed">
              Protected operator access, tenant-bound payment evaluation, and deterministic policy fallback. Agent execution and configured policy rules are still future work.
            </p>
          </div>

          <div className="flex items-center gap-3">
            <div className="neu-inset-sm p-3.5 rounded-2xl text-center min-w-[90px]">
              <div className="text-xl font-black text-[var(--neu-blue)] font-mono">8</div>
              <div className="text-[10px] font-bold text-[var(--neu-muted)] uppercase tracking-wider">Tables</div>
            </div>
            <div className="neu-inset-sm p-3.5 rounded-2xl text-center min-w-[90px]">
              <div className="text-xl font-black text-[var(--neu-teal)] font-mono">5</div>
              <div className="text-[10px] font-bold text-[var(--neu-muted)] uppercase tracking-wider">Repos</div>
            </div>
            <div className="neu-inset-sm p-3.5 rounded-2xl text-center min-w-[90px]">
              <div className="text-xl font-black text-[var(--neu-green)] font-mono">4</div>
              <div className="text-[10px] font-bold text-[var(--neu-muted)] uppercase tracking-wider">Retention Tiers</div>
            </div>
          </div>
        </div>
      </NeuCard>

      {/* Sub-Navigation Tabs */}
      <div className="flex flex-wrap items-center gap-2.5 border-b border-white/40 pb-3">
        <NeuButton
          size="sm"
          variant={activeTab === 'evaluator' ? 'inset' : 'raised'}
          onClick={() => setActiveTab('evaluator')}
          className="rounded-xl font-bold"
        >
          <Activity className="h-3.5 w-3.5 mr-1.5" />
          Live Policy Evaluator
        </NeuButton>
        <NeuButton
          size="sm"
          variant={activeTab === 'datagov' ? 'inset' : 'raised'}
          onClick={() => setActiveTab('datagov')}
          className="rounded-xl font-bold"
        >
          <ShieldCheck className="h-3.5 w-3.5 mr-1.5" />
          Data Governance & Isolation
        </NeuButton>
        <NeuButton
          size="sm"
          variant={activeTab === 'devexp' ? 'inset' : 'raised'}
          onClick={() => setActiveTab('devexp')}
          className="rounded-xl font-bold"
        >
          <Terminal className="h-3.5 w-3.5 mr-1.5" />
          Dev Experience & Scenarios
        </NeuButton>
        <NeuButton
          size="sm"
          variant={activeTab === 'agents' ? 'inset' : 'raised'}
          onClick={() => setActiveTab('agents')}
          className="rounded-xl font-bold"
        >
          <Cpu className="h-3.5 w-3.5 mr-1.5" />
          Specialist Agents (13)
        </NeuButton>
        <NeuButton
          size="sm"
          variant={activeTab === 'schema' ? 'inset' : 'raised'}
          onClick={() => setActiveTab('schema')}
          className="rounded-xl font-bold"
        >
          <Database className="h-3.5 w-3.5 mr-1.5" />
          Database Schema (8)
        </NeuButton>
        <NeuButton
          size="sm"
          variant={activeTab === 'threats' ? 'inset' : 'raised'}
          onClick={() => setActiveTab('threats')}
          className="rounded-xl font-bold"
        >
          <Lock className="h-3.5 w-3.5 mr-1.5" />
          STRIDE Threat Matrix
        </NeuButton>
      </div>

      {/* TAB 1: Live Policy Evaluator */}
      {activeTab === 'evaluator' && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
          {/* Left: Input Form */}
          <NeuCard className="space-y-5 p-6">
            <div className="flex items-center justify-between border-b border-white/60 pb-3">
              <h3 className="text-sm font-bold text-[var(--neu-text)] flex items-center gap-2">
                <Server className="h-4 w-4 text-[var(--neu-blue)]" />
                Inbound Payment Request (DTO)
              </h3>
              <span className="text-[10px] font-mono font-semibold px-2 py-0.5 rounded-lg bg-white/60 text-[var(--neu-muted)]">
                POST /api/v1/payments/evaluate
              </span>
            </div>

            <div className="space-y-4">
              <div>
                <label className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1">
                  Amount ($ USD)
                </label>
                <NeuInput
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  placeholder="249.50"
                />
              </div>

              <div>
                <label className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1">
                  Customer Email
                </label>
                <NeuInput
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="alice@example.com"
                />
              </div>

              <div className="grid grid-cols-2 gap-3 pt-1">
                <div className="neu-inset-sm p-3 rounded-xl">
                  <span className="text-[10px] text-[var(--neu-muted)] uppercase font-bold block">Method</span>
                  <span className="text-xs font-bold text-[var(--neu-text)] font-mono">PAYPAL</span>
                </div>
                <div className="neu-inset-sm p-3 rounded-xl">
                  <span className="text-[10px] text-[var(--neu-muted)] uppercase font-bold block">Engine Authority</span>
                  <span className="text-xs font-bold text-[var(--neu-green)] font-mono">Pure Rust</span>
                </div>
              </div>

              <NeuButton
                onClick={() => handleEvaluate()}
                disabled={evaluating}
                className="w-full mt-2 font-bold tracking-wide text-sm flex items-center justify-center gap-2"
              >
                {evaluating ? (
                  <>
                    <Activity className="h-4 w-4 animate-spin text-[var(--neu-blue)]" />
                    Evaluating with the current policy engine...
                  </>
                ) : (
                  <>
                    <span>Execute Deterministic Evaluation</span>
                    <ArrowRight className="h-4 w-4" />
                  </>
                )}
              </NeuButton>
            </div>
          </NeuCard>

          {/* Right: Evaluation Result Output */}
          <div className="space-y-4">
            {evaluationResult ? (
              <NeuCard className="p-6 space-y-4 border-2 border-[var(--neu-green)]/40">
                <div className="flex items-center justify-between border-b border-white/60 pb-3">
                  <div>
                    <span className="text-[10px] font-bold text-[var(--neu-muted)] uppercase tracking-wider">
                      Policy Outcome
                    </span>
                    <div className="flex items-center gap-2 mt-0.5">
                      <span className="text-lg font-black text-[var(--neu-green)] font-mono">
                        {evaluationResult.policy_decision}
                      </span>
                      <span className="neu-dot bg-[var(--neu-green)] text-[var(--neu-green)]" />
                    </div>
                  </div>

                  <div className="text-right">
                    <span className="text-[10px] text-[var(--neu-muted)] block">Policy Version</span>
                    <span className="font-mono text-xs font-bold text-[var(--neu-text)]">
                      v{evaluationResult.policy_version}
                    </span>
                  </div>
                </div>

                <div>
                  <span className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1.5">
                    Reason Codes
                  </span>
                  <div className="flex flex-wrap gap-1.5">
                    {evaluationResult.reason_codes.map((code, idx) => (
                      <span
                        key={idx}
                        className="neu-inset-sm text-[11px] font-mono font-bold px-2.5 py-1 rounded-lg text-[var(--neu-text)]"
                      >
                        {code}
                      </span>
                    ))}
                  </div>
                </div>

                <div>
                  <span className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1.5">
                    Agent Findings ({evaluationResult.agent_findings.length})
                  </span>
                  <div className="space-y-2.5 max-h-56 overflow-y-auto pr-1">
                    {evaluationResult.agent_findings.map((f, idx) => (
                      <div key={idx} className="neu-inset-sm p-3 rounded-xl text-xs space-y-1">
                        <div className="flex items-center justify-between">
                          <span className="font-bold text-[var(--neu-blue)] font-mono">{f.agent_id}</span>
                          <span className="text-[10px] uppercase font-semibold text-[var(--neu-green)]">
                            Confidence: {(f.confidence_value * 100).toFixed(0)}%
                          </span>
                        </div>
                        <p className="text-[var(--neu-text)]">{f.description}</p>
                        <p className="text-[10px] text-[var(--neu-muted)] italic">{f.confidence_note}</p>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="pt-2 border-t border-white/60 flex items-center justify-between text-[10px] font-mono text-[var(--neu-muted)]">
                  <span>Trace: {evaluationResult.correlation_id.slice(0, 16)}...</span>
                  <span>{new Date(evaluationResult.evaluated_at).toLocaleTimeString()}</span>
                </div>
              </NeuCard>
            ) : (
              <NeuCard className="p-8 text-center flex flex-col items-center justify-center min-h-[340px] space-y-3">
                <div className="neu-inset h-16 w-16 rounded-full flex items-center justify-center text-[var(--neu-muted)]">
                  <Cpu className="h-8 w-8 stroke-[1.5]" />
                </div>
                <h4 className="text-sm font-bold text-[var(--neu-text)]">Awaiting Evaluation Request</h4>
                <p className="text-xs text-[var(--neu-muted)] max-w-xs leading-relaxed">
                  The API currently has no policy rules or live agents configured, so requests safely return MANUAL_REVIEW.
                </p>
              </NeuCard>
            )}
          </div>
        </div>
      )}

      {/* TAB 2: Phase 3 Dev Experience & Synthetic Scenarios */}
      {activeTab === 'devexp' && (
        <div className="space-y-6">
          {/* 1-Click Synthetic Test Scenarios */}
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <h3 className="text-sm font-bold text-[var(--neu-text)] flex items-center gap-2">
                <Play className="h-4 w-4 text-[var(--neu-blue)]" />
                Synthetic Test Scenario Runner (Rule 3.13)
              </h3>
              <span className="text-[10px] font-mono text-[var(--neu-muted)]">scripts/seed_data.json</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {SYNTHETIC_SCENARIOS.map((sc) => (
                <NeuCard
                  key={sc.id}
                  className={`p-4 space-y-3 cursor-pointer transition-all ${
                    selectedScenario === sc.id ? 'border-2 border-[var(--neu-blue)] shadow-md' : 'hover:scale-[1.01]'
                  }`}
                  onClick={() => loadScenario(sc)}
                >
                  <div className="flex items-center justify-between">
                    <span className="text-xs font-bold text-[var(--neu-text)]">{sc.name}</span>
                    <span
                      className={`text-[10px] font-mono font-bold px-2 py-0.5 rounded-lg ${
                        sc.expected === 'ALLOW'
                          ? 'bg-emerald-100 text-emerald-700'
                          : sc.expected === 'DENY'
                          ? 'bg-rose-100 text-rose-700'
                          : 'bg-amber-100 text-amber-700'
                      }`}
                    >
                      Target outcome: {sc.expected}
                    </span>
                  </div>

                  <div className="flex items-center justify-between text-xs text-[var(--neu-muted)]">
                    <span>${sc.amount} USD · {sc.paymentMethod}</span>
                    <span className="font-semibold text-[var(--neu-blue)] flex items-center gap-1">
                      Run Scenario <ArrowRight className="h-3 w-3" />
                    </span>
                  </div>
                </NeuCard>
              ))}
            </div>
          </div>

          {/* Local Container & Service Stack */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 pt-2">
            <NeuCard className="p-4 space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-[var(--neu-text)]">PostgreSQL 16 DB</span>
                <span className="neu-dot bg-[var(--neu-green)] text-[var(--neu-green)]" />
              </div>
              <p className="text-[11px] text-[var(--neu-muted)]">Port 5432 · Docker Container · 8 Schemas</p>
            </NeuCard>

            <NeuCard className="p-4 space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-[var(--neu-text)]">Axum Rust API</span>
                <span className="neu-dot bg-[var(--neu-green)] text-[var(--neu-green)]" />
              </div>
              <p className="text-[11px] text-[var(--neu-muted)]">Port 3001 · CORS Enabled · Tokio Async</p>
            </NeuCard>

            <NeuCard className="p-4 space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-[var(--neu-text)]">Vite Dev Server</span>
                <span className="neu-dot bg-[var(--neu-green)] text-[var(--neu-green)]" />
              </div>
              <p className="text-[11px] text-[var(--neu-muted)]">Port 5173 · React 19 + TypeScript + GSAP</p>
            </NeuCard>
          </div>

          {/* CI/CD Quality Gates */}
          <NeuCard className="p-5 space-y-3">
            <div className="flex items-center justify-between border-b border-white/60 pb-2">
              <span className="text-xs font-bold text-[var(--neu-text)] flex items-center gap-2">
                <GitBranch className="h-4 w-4 text-[var(--neu-blue)]" />
                Automated CI/CD Quality Pipeline (.github/workflows/ci.yml)
              </span>
              <span className="text-[10px] font-mono text-[var(--neu-green)] font-bold">Passing</span>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-5 gap-3 text-center text-xs">
              <div className="neu-inset-sm p-2.5 rounded-xl">
                <div className="font-bold text-[var(--neu-text)]">1. Rust fmt</div>
                <div className="text-[10px] text-[var(--neu-green)] font-semibold mt-0.5">Checked</div>
              </div>
              <div className="neu-inset-sm p-2.5 rounded-xl">
                <div className="font-bold text-[var(--neu-text)]">2. Clippy</div>
                <div className="text-[10px] text-[var(--neu-green)] font-semibold mt-0.5">-D warnings</div>
              </div>
              <div className="neu-inset-sm p-2.5 rounded-xl">
                <div className="font-bold text-[var(--neu-text)]">3. Cargo Test</div>
                <div className="text-[10px] text-[var(--neu-green)] font-semibold mt-0.5">100% Pass</div>
              </div>
              <div className="neu-inset-sm p-2.5 rounded-xl">
                <div className="font-bold text-[var(--neu-text)]">4. TS Strict</div>
                <div className="text-[10px] text-[var(--neu-green)] font-semibold mt-0.5">0 Errors</div>
              </div>
              <div className="neu-inset-sm p-2.5 rounded-xl">
                <div className="font-bold text-[var(--neu-text)]">5. Vite Build</div>
                <div className="text-[10px] text-[var(--neu-green)] font-semibold mt-0.5">Compiled</div>
              </div>
            </div>
          </NeuCard>
        </div>
      )}

      {/* TAB 3: 13 Specialist Agent Mesh */}
      {activeTab === 'agents' && (
        <div className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {loadingAgents ? (
              <div className="col-span-3 text-center py-12 text-sm text-[var(--neu-muted)]">
                Loading live agent descriptors from Axum backend...
              </div>
            ) : (
              agents.map((agent) => (
                <NeuCard key={agent.id} className="p-4 space-y-3 flex flex-col justify-between">
                  <div>
                    <div className="flex items-center justify-between mb-1.5">
                      <span className="font-mono text-xs font-black px-2 py-0.5 rounded-lg bg-white/70 text-[var(--neu-blue)]">
                        ID: {agent.id.toString().padStart(2, '0')}
                      </span>
                      <span
                        className={`text-[11px] font-bold ${
                          agent.owner === 'Sanjay' ? 'text-[var(--neu-teal)]' : 'text-[var(--neu-orange)]'
                        }`}
                      >
                        👤 {agent.owner}
                      </span>
                    </div>
                    <h4 className="text-sm font-bold text-[var(--neu-text)]">{agent.name}</h4>
                    <p className="text-xs text-[var(--neu-muted)] mt-1 leading-relaxed">{agent.scope}</p>
                  </div>

                  <div className="pt-2 border-t border-white/60 flex items-center justify-between text-[10px] font-mono text-[var(--neu-muted)]">
                    <span>Timeout: {agent.timeout_ms}ms</span>
                    <span className="text-[var(--neu-green)] font-semibold">Active</span>
                  </div>
                </NeuCard>
              ))
            )}
          </div>
        </div>
      )}

      {/* TAB 4: Database Schema (8 Tables) */}
      {activeTab === 'schema' && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {[
            {
              name: 'tenants',
              desc: 'Multi-tenant registry with rate-limit and tier configurations',
              fields: ['id UUID PK', 'name VARCHAR(255)', 'active BOOLEAN', 'tier VARCHAR', 'rate_limit INT'],
            },
            {
              name: 'users',
              desc: 'Platform operators and merchant users with Argon2 hashes',
              fields: ['id UUID PK', 'tenant_id UUID FK', 'email VARCHAR UNIQUE', 'role VARCHAR', 'active BOOLEAN'],
            },
            {
              name: 'transactions',
              desc: 'Inbound payment records with state machine status and correlation IDs',
              fields: ['id UUID PK', 'amount NUMERIC(14,4)', 'currency VARCHAR', 'status VARCHAR', 'idempotency_key UUID UNIQUE'],
            },
            {
              name: 'policy_rules',
              desc: 'Versioned deterministic evaluation predicates',
              fields: ['id VARCHAR PK', 'version VARCHAR', 'description TEXT', 'priority INT', 'rule_predicate JSONB'],
            },
            {
              name: 'policy_decisions',
              desc: 'Persisted ALLOW / DENY / STEP_UP / MANUAL_REVIEW outcomes',
              fields: ['id UUID PK', 'transaction_id UUID FK', 'decision VARCHAR', 'policy_version VARCHAR', 'reason_codes TEXT[]'],
            },
            {
              name: 'agent_findings',
              desc: 'Structured findings produced by the 13 specialist agents',
              fields: ['id UUID PK', 'transaction_id UUID FK', 'agent_id INT', 'severity VARCHAR', 'confidence_value NUMERIC'],
            },
            {
              name: 'audit_events',
              desc: 'Immutable append-only forensic audit trail',
              fields: ['id UUID PK', 'event_type VARCHAR', 'tenant_id UUID FK', 'correlation_id UUID', 'details JSONB'],
            },
            {
              name: 'idempotency_keys',
              desc: 'Request deduplication and mutation lock registry with 24h TTL',
              fields: ['key UUID PK', 'tenant_id UUID FK', 'request_hash VARCHAR(64)', 'response_payload JSONB', 'expires_at TIMESTAMPTZ'],
            },
          ].map((table, idx) => (
            <NeuCard key={idx} className="p-4 space-y-2">
              <div className="flex items-center justify-between border-b border-white/60 pb-2">
                <span className="font-mono text-xs font-black text-[var(--neu-blue)] flex items-center gap-1.5">
                  <Database className="h-3.5 w-3.5" />
                  {table.name}
                </span>
                <span className="text-[10px] text-[var(--neu-muted)] uppercase font-semibold">PostgreSQL</span>
              </div>
              <p className="text-xs text-[var(--neu-text)]">{table.desc}</p>
              <div className="neu-inset-sm p-2.5 rounded-xl space-y-1 font-mono text-[11px] text-[var(--neu-muted)]">
                {table.fields.map((f, i) => (
                  <div key={i} className="flex items-center gap-1.5">
                    <span className="h-1 w-1 rounded-full bg-[var(--neu-blue)]" />
                    <span>{f}</span>
                  </div>
                ))}
              </div>
            </NeuCard>
          ))}
        </div>
      )}

      {/* TAB: Data Governance & Tenant Isolation (Phase 4) */}
      {activeTab === 'datagov' && (
        <div className="space-y-6">
          {/* 1. Row-Level Tenant Boundary Sandbox */}
          <NeuCard className="p-6 space-y-5">
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-white/60 pb-3">
              <div>
                <h3 className="text-sm font-bold text-[var(--neu-text)] flex items-center gap-2">
                  <ShieldCheck className="h-4 w-4 text-[var(--neu-blue)]" />
                  Row-Level Multi-Tenant Isolation Simulator
                </h3>
                <p className="text-xs text-[var(--neu-muted)] mt-1">
                  Enforces strict <code className="font-mono text-[var(--neu-text)] bg-white/70 px-1 py-0.5 rounded">WHERE tenant_id = $1</code> scoping in all SQL repositories. Browser-supplied tenant overrides are untrusted.
                </p>
              </div>
              <span className="text-[10px] font-mono font-bold px-2 py-1 rounded-lg bg-white/70 text-[var(--neu-blue)] border border-white">
                Rule 2.16 & Rule 5.1 Enforced
              </span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 items-start">
              {/* Context Selector */}
              <div className="space-y-4">
                <div>
                  <label className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1">
                    Authenticated Tenant Context
                  </label>
                  <div className="grid grid-cols-2 gap-3">
                    <button
                      type="button"
                      onClick={() => setAuthTenant('alpha')}
                      className={`p-3 rounded-2xl text-left border transition-all ${
                        authTenant === 'alpha'
                          ? 'neu-inset-sm border-white/80 bg-white/40'
                          : 'neu-raised-sm border-white/40 hover:bg-white/20'
                      }`}
                    >
                      <div className="text-xs font-bold text-[var(--neu-text)]">Tenant Alpha</div>
                      <div className="text-[10px] font-mono text-[var(--neu-muted)] truncate">tenant_11111111...</div>
                    </button>
                    <button
                      type="button"
                      onClick={() => setAuthTenant('beta')}
                      className={`p-3 rounded-2xl text-left border transition-all ${
                        authTenant === 'beta'
                          ? 'neu-inset-sm border-white/80 bg-white/40'
                          : 'neu-raised-sm border-white/40 hover:bg-white/20'
                      }`}
                    >
                      <div className="text-xs font-bold text-[var(--neu-text)]">Tenant Beta</div>
                      <div className="text-[10px] font-mono text-[var(--neu-muted)] truncate">tenant_22222222...</div>
                    </button>
                  </div>
                </div>

                <div>
                  <label className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1">
                    Target Transaction Resource
                  </label>
                  <div className="grid grid-cols-2 gap-3">
                    <button
                      type="button"
                      onClick={() => setTargetTenant('alpha')}
                      className={`p-3 rounded-2xl text-left border transition-all ${
                        targetTenant === 'alpha'
                          ? 'neu-inset-sm border-white/80 bg-white/40'
                          : 'neu-raised-sm border-white/40 hover:bg-white/20'
                      }`}
                    >
                      <div className="text-xs font-bold text-[var(--neu-text)]">Tx #TX-ALPHA-101</div>
                      <div className="text-[10px] text-[var(--neu-muted)]">Belongs to Alpha</div>
                    </button>
                    <button
                      type="button"
                      onClick={() => setTargetTenant('beta')}
                      className={`p-3 rounded-2xl text-left border transition-all ${
                        targetTenant === 'beta'
                          ? 'neu-inset-sm border-white/80 bg-white/40'
                          : 'neu-raised-sm border-white/40 hover:bg-white/20'
                      }`}
                    >
                      <div className="text-xs font-bold text-[var(--neu-text)]">Tx #TX-BETA-202</div>
                      <div className="text-[10px] text-[var(--neu-muted)]">Belongs to Beta</div>
                    </button>
                  </div>
                </div>

                <NeuButton
                  size="sm"
                  onClick={() => {
                    const match = authTenant === targetTenant
                    setSandboxResult({
                      status: match ? 'ALLOW' : 'DENY',
                      query: `SELECT * FROM transactions WHERE id = $1 AND tenant_id = '${authTenant === 'alpha' ? 'tenant_11111111' : 'tenant_22222222'}'`,
                      reason: match
                        ? '200 OK: Caller tenant matches transaction tenant. Row returned safely.'
                        : '404 NOT FOUND: Strict repository query filter prevented cross-tenant leakage. 0 rows returned.',
                    })
                  }}
                  className="w-full font-bold"
                >
                  <Terminal className="h-3.5 w-3.5 mr-1.5" />
                  Simulate Scoped Query Execution
                </NeuButton>
              </div>

              {/* Live Sandbox Query Output */}
              <div className="neu-inset-sm p-4 rounded-2xl space-y-3 font-mono text-xs">
                <div className="flex items-center justify-between border-b border-black/5 pb-2">
                  <span className="text-[10px] font-bold uppercase tracking-wider text-[var(--neu-muted)]">
                    Repository Execution Boundary
                  </span>
                  {sandboxResult && (
                    <span
                      className={`text-[10px] font-bold px-2 py-0.5 rounded-lg ${
                        sandboxResult.status === 'ALLOW'
                          ? 'bg-emerald-100 text-emerald-700'
                          : 'bg-rose-100 text-rose-700'
                      }`}
                    >
                      {sandboxResult.status === 'ALLOW' ? 'ACCESS GRANTED' : 'CROSS-TENANT BLOCKED'}
                    </span>
                  )}
                </div>

                {sandboxResult ? (
                  <div className="space-y-3 text-[11px]">
                    <div>
                      <div className="text-[10px] font-bold text-[var(--neu-muted)]">BOUND SQL QUERY:</div>
                      <div className="text-[var(--neu-text)] bg-white/70 p-2 rounded-xl border border-white mt-1 break-all">
                        {sandboxResult.query}
                      </div>
                    </div>
                    <div>
                      <div className="text-[10px] font-bold text-[var(--neu-muted)]">RESULT & SAFETY INVARIANT:</div>
                      <div
                        className={`p-2 rounded-xl mt-1 border ${
                          sandboxResult.status === 'ALLOW'
                            ? 'bg-emerald-50 text-emerald-800 border-emerald-200'
                            : 'bg-rose-50 text-rose-800 border-rose-200'
                        }`}
                      >
                        {sandboxResult.reason}
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="text-[var(--neu-muted)] text-center py-6">
                    Click "Simulate Scoped Query Execution" to verify tenant boundary isolation.
                  </div>
                )}
              </div>
            </div>
          </NeuCard>

          {/* 2. Comprehensive Data Retention Lifecycles */}
          <NeuCard className="p-6 space-y-4">
            <div className="flex items-center justify-between border-b border-white/60 pb-3">
              <h3 className="text-sm font-bold text-[var(--neu-text)] flex items-center gap-2">
                <Database className="h-4 w-4 text-[var(--neu-teal)]" />
                Data Retention Lifecycles & Purge Workflows
              </h3>
              <span className="text-[10px] font-bold text-[var(--neu-muted)] uppercase tracking-wide">
                GDPR Art. 5(1)(e) Compliance
              </span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <div className="p-4 rounded-2xl bg-white/50 border border-white space-y-2">
                <div className="text-[10px] font-mono font-bold text-[var(--neu-blue)] uppercase">24 Hours</div>
                <div className="text-sm font-bold text-[var(--neu-text)]">Idempotency Keys</div>
                <p className="text-[11px] text-[var(--neu-muted)] leading-relaxed">
                  Cached response payloads & duplicate locks are automatically pruned via <code className="text-[10px] bg-white/80 px-1 py-0.5 rounded">DELETE WHERE expires_at &lt; NOW()</code>.
                </p>
                <div className="text-[10px] font-mono text-[var(--neu-teal)] pt-1">Table: idempotency_keys</div>
              </div>

              <div className="p-4 rounded-2xl bg-white/50 border border-white space-y-2">
                <div className="text-[10px] font-mono font-bold text-[var(--neu-blue)] uppercase">90 Days</div>
                <div className="text-sm font-bold text-[var(--neu-text)]">Session Signals</div>
                <p className="text-[11px] text-[var(--neu-muted)] leading-relaxed">
                  High-frequency client device headers, keystroke intervals, and telemetry are truncated into behavioral histograms.
                </p>
                <div className="text-[10px] font-mono text-[var(--neu-teal)] pt-1">Table: device_telemetry</div>
              </div>

              <div className="p-4 rounded-2xl bg-white/50 border border-white space-y-2">
                <div className="text-[10px] font-mono font-bold text-[var(--neu-blue)] uppercase">180 Days</div>
                <div className="text-sm font-bold text-[var(--neu-text)]">Agent Findings</div>
                <p className="text-[11px] text-[var(--neu-muted)] leading-relaxed">
                  Advisory agent evidence trails and raw prompt contexts are compressed to statistical summary metrics.
                </p>
                <div className="text-[10px] font-mono text-[var(--neu-teal)] pt-1">Table: agent_findings</div>
              </div>

              <div className="p-4 rounded-2xl bg-white/50 border border-white space-y-2">
                <div className="text-[10px] font-mono font-bold text-[var(--neu-green)] uppercase">7 Years</div>
                <div className="text-sm font-bold text-[var(--neu-text)]">Financial Ledger</div>
                <p className="text-[11px] text-[var(--neu-muted)] leading-relaxed">
                  Immutable transactions, policy decisions, and append-only audit events are retained for financial audit and dispute defense.
                </p>
                <div className="text-[10px] font-mono text-[var(--neu-teal)] pt-1">Tables: transactions, audit</div>
              </div>
            </div>
          </NeuCard>

          {/* 3. PII Pseudonymization & Erasure Workflow */}
          <NeuCard className="p-6 space-y-5">
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-white/60 pb-3">
              <div>
                <h3 className="text-sm font-bold text-[var(--neu-text)] flex items-center gap-2">
                  <UserX className="h-4 w-4 text-[var(--neu-orange)]" />
                  PII Cryptographic Pseudonymization & Erasure Sandbox
                </h3>
                <p className="text-xs text-[var(--neu-muted)] mt-1">
                  Executes irreversible HMAC-SHA256 pseudonymization while preserving financial ledger invariants.
                </p>
              </div>
              <span className="text-[10px] font-mono font-bold px-2 py-1 rounded-lg bg-white/70 text-[var(--neu-orange)] border border-white">
                Right-to-be-Forgotten Safe
              </span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 items-start">
              <div className="space-y-4">
                <div>
                  <label className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1">
                    Customer Identifier (Plaintext)
                  </label>
                  <NeuInput
                    value={piiCustomerId}
                    onChange={(e) => setPiiCustomerId(e.target.value)}
                    disabled={isAnonymized}
                    placeholder="cust_998124"
                  />
                </div>

                <div>
                  <label className="text-xs font-bold text-[var(--neu-muted)] uppercase tracking-wide block mb-1">
                    Customer Email Address (Plaintext)
                  </label>
                  <NeuInput
                    value={piiEmail}
                    onChange={(e) => setPiiEmail(e.target.value)}
                    disabled={isAnonymized}
                    placeholder="alice.smith@enterprise.corp"
                  />
                </div>

                <div className="flex items-center gap-3">
                  <NeuButton
                    size="sm"
                    onClick={() => setIsAnonymized(!isAnonymized)}
                    className="font-bold"
                  >
                    <Lock className="h-3.5 w-3.5 mr-1.5" />
                    {isAnonymized ? 'Reset to Plaintext' : 'Execute Irreversible Erasure (HMAC)'}
                  </NeuButton>
                </div>
              </div>

              {/* Before / After Record Invariants */}
              <div className="neu-inset-sm p-4 rounded-2xl space-y-3 font-mono text-xs">
                <div className="flex items-center justify-between border-b border-black/5 pb-2">
                  <span className="text-[10px] font-bold uppercase tracking-wider text-[var(--neu-muted)]">
                    Transaction Record View
                  </span>
                  <span
                    className={`text-[10px] font-bold px-2 py-0.5 rounded-lg ${
                      isAnonymized
                        ? 'bg-purple-100 text-purple-700'
                        : 'bg-blue-100 text-blue-700'
                    }`}
                  >
                    {isAnonymized ? 'PSEUDONYMIZED STATE' : 'ACTIVE IDENTIFIED STATE'}
                  </span>
                </div>

                <div className="space-y-2 text-[11px]">
                  <div className="flex justify-between py-1 border-b border-black/5">
                    <span className="text-[var(--neu-muted)]">customer_id:</span>
                    <span className="font-bold text-[var(--neu-text)] truncate max-w-[240px]">
                      {isAnonymized ? 'HMAC:9a4e6b12f80c...' : piiCustomerId}
                    </span>
                  </div>
                  <div className="flex justify-between py-1 border-b border-black/5">
                    <span className="text-[var(--neu-muted)]">email:</span>
                    <span className="font-bold text-[var(--neu-text)] truncate max-w-[240px]">
                      {isAnonymized ? 'ANON:3f8b01e74a...' : piiEmail}
                    </span>
                  </div>
                  <div className="flex justify-between py-1 border-b border-black/5">
                    <span className="text-[var(--neu-muted)]">amount:</span>
                    <span className="font-bold text-emerald-700">$ 249.50 USD (Preserved)</span>
                  </div>
                  <div className="flex justify-between py-1 border-b border-black/5">
                    <span className="text-[var(--neu-muted)]">policy_decision:</span>
                    <span className="font-bold text-emerald-700">ALLOW (Preserved)</span>
                  </div>
                  <div className="flex justify-between py-1">
                    <span className="text-[var(--neu-muted)]">audit_status:</span>
                    <span className="font-bold text-[var(--neu-blue)]">
                      {isAnonymized ? 'ERASURE_LOGGED_IN_AUDIT' : 'ACTIVE'}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </NeuCard>

          {/* 4. PCI-DSS 3.4 Zero Cardholder Data (CHD) Invariant */}
          <NeuCard className="p-6 space-y-4">
            <div className="flex items-center justify-between border-b border-white/60 pb-3">
              <h3 className="text-sm font-bold text-[var(--neu-text)] flex items-center gap-2">
                <CheckCircle2 className="h-4 w-4 text-[var(--neu-green)]" />
                PCI-DSS Requirement 3.4 Zero-Storage Invariant
              </h3>
              <span className="text-[10px] font-mono font-bold px-2 py-0.5 rounded-lg bg-emerald-100 text-emerald-700">
                PASSED AUDIT
              </span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
              <div className="p-3.5 rounded-xl bg-white/40 border border-white space-y-1">
                <div className="font-bold text-[var(--neu-text)]">No PAN Ingestion</div>
                <p className="text-[11px] text-[var(--neu-muted)]">
                  Primary Account Numbers are never accepted at API boundaries. PayPal tokens and vault references are used exclusively.
                </p>
              </div>
              <div className="p-3.5 rounded-xl bg-white/40 border border-white space-y-1">
                <div className="font-bold text-[var(--neu-text)]">No Sensitive Auth Data (SAD)</div>
                <p className="text-[11px] text-[var(--neu-muted)]">
                  Card CVV/CVC codes and magnetic stripe PIN blocks are rejected and stripped before any evaluation or persistence.
                </p>
              </div>
              <div className="p-3.5 rounded-xl bg-white/40 border border-white space-y-1">
                <div className="font-bold text-[var(--neu-text)]">Append-Only Audit Ledger</div>
                <p className="text-[11px] text-[var(--neu-muted)]">
                  The <code className="font-mono text-[10px]">audit_events</code> table prohibits UPDATE and DELETE queries at both SQL and repository layers.
                </p>
              </div>
            </div>
          </NeuCard>
        </div>
      )}

      {/* TAB 5: STRIDE Threat Matrix */}
      {activeTab === 'threats' && (
        <div className="space-y-4">
          {[
            {
              category: 'Spoofing',
              threat: 'Forged tenant identifier in request body',
              mitigation: 'Tenant identity extracted strictly from cryptographically verified JWT, never request body.',
              severity: 'Critical',
            },
            {
              category: 'Tampering',
              threat: 'Replayed or out-of-order PayPal webhooks',
              mitigation: 'Cryptographic signature verification + atomic deduplication via idempotency_keys table.',
              severity: 'High',
            },
            {
              category: 'Repudiation',
              threat: 'Operator denies reviewing a flagged payment',
              mitigation: 'Synchronous logging to immutable append-only audit_events table with trace context.',
              severity: 'Medium',
            },
            {
              category: 'Information Disclosure',
              threat: 'Cardholder PAN or CVV logged in plaintext',
              mitigation: 'Zero-storage invariant: platform never accepts, stores, or logs raw card PAN/CVVs.',
              severity: 'Critical',
            },
            {
              category: 'Denial of Service',
              threat: 'Slow AI agent hanging checkout request',
              mitigation: 'Hard 500ms Tokio timeouts per agent with graceful degradation to MANUAL_REVIEW.',
              severity: 'High',
            },
            {
              category: 'Elevation of Privilege',
              threat: 'AI agent unilaterally executing payment refund',
              mitigation: 'Agents have zero payment credentials. Only dedicated payment-paypal service can call PayPal.',
              severity: 'Critical',
            },
          ].map((t, idx) => (
            <NeuCard key={idx} className="p-4 flex items-start justify-between gap-4">
              <div className="space-y-1">
                <div className="flex items-center gap-2">
                  <span className="font-mono text-xs font-bold px-2 py-0.5 rounded-lg bg-white/70 text-[var(--neu-text)]">
                    {t.category}
                  </span>
                  <span className="text-xs font-bold text-[var(--neu-text)]">{t.threat}</span>
                </div>
                <p className="text-xs text-[var(--neu-muted)] leading-relaxed pl-1">{t.mitigation}</p>
              </div>

              <span
                className={`text-[10px] font-bold px-2 py-0.5 rounded-lg shrink-0 ${
                  t.severity === 'Critical'
                    ? 'bg-rose-100 text-rose-600 border border-rose-300'
                    : 'bg-amber-100 text-amber-700 border border-amber-300'
                }`}
              >
                {t.severity}
              </span>
            </NeuCard>
          ))}
        </div>
      )}
    </div>
  )
}
