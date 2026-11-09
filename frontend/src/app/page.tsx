import { useRef, useState } from 'react'
import { Lock, ShieldCheck, Palette } from 'lucide-react'
import {
  NeuCard,
  NeuButton,
  NeuInput,
  NeuSwitch,
  NeuIconButton,
  NeuLikeButton,
  SparklineCard,
  BalanceCard,
  CreditCard,
  RingChart,
  LegendRows,
  PaginationDots,
} from '@/components/neu'
import { gsap, useGSAP } from '@/lib/gsap'
import Phase2Architecture from './Phase2Architecture'

export default function NeumorphismPage() {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const [viewMode, setViewMode] = useState<'architecture' | 'uikit'>('architecture')
  const [notificationActive, setNotificationActive] = useState(true)
  const [miniSwitchActive, setMiniSwitchActive] = useState(true)
  const [paginationIndex, setPaginationIndex] = useState(0)

  // GSAP Staggered Entrance Animation with Reduced Motion Support
  useGSAP(
    () => {
      const mm = gsap.matchMedia()

      mm.add('(prefers-reduced-motion: no-preference)', () => {
        gsap.from('.neu-animate-entry', {
          y: 20,
          opacity: 0,
          duration: 0.7,
          stagger: 0.05,
          ease: 'power3.out',
        })
      })

      return () => mm.revert()
    },
    { scope: containerRef, dependencies: [viewMode] }
  )

  return (
    <div
      ref={containerRef}
      className="min-h-screen w-full bg-[var(--neu-bg)] text-[var(--neu-text)] flex flex-col items-center p-6 md:p-10 selection:bg-[var(--neu-blue)] selection:text-white"
    >
      {/* Top Header Bar */}
      <header className="w-full max-w-5xl mb-6 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-xl font-black tracking-tight text-[var(--neu-text)]">
              Payment Security Platform
            </h1>
            <span className="text-[10px] font-bold uppercase tracking-wider px-2 py-0.5 rounded-full bg-white/70 border border-white text-[var(--neu-blue)]">
              Phase 2 Active
            </span>
          </div>
          <p className="text-xs text-[var(--neu-muted)] mt-0.5">
            Multi-Agent Intelligence · Rust Core Engine · Neumorphic (Soft UI) Theme
          </p>
        </div>

        {/* View Switcher + Team Badges */}
        <div className="flex items-center gap-3">
          <div className="neu-inset-sm p-1 rounded-2xl flex items-center gap-1">
            <button
              onClick={() => setViewMode('architecture')}
              className={`px-3 py-1.5 rounded-xl text-xs font-bold transition-all cursor-pointer flex items-center gap-1.5 ${
                viewMode === 'architecture'
                  ? 'neu-raised bg-[var(--neu-bg)] text-[var(--neu-blue)]'
                  : 'text-[var(--neu-muted)] hover:text-[var(--neu-text)]'
              }`}
            >
              <ShieldCheck className="h-3.5 w-3.5" />
              <span>Phase 2 Architecture</span>
            </button>
            <button
              onClick={() => setViewMode('uikit')}
              className={`px-3 py-1.5 rounded-xl text-xs font-bold transition-all cursor-pointer flex items-center gap-1.5 ${
                viewMode === 'uikit'
                  ? 'neu-raised bg-[var(--neu-bg)] text-[var(--neu-blue)]'
                  : 'text-[var(--neu-muted)] hover:text-[var(--neu-text)]'
              }`}
            >
              <Palette className="h-3.5 w-3.5" />
              <span>UI Kit Components</span>
            </button>
          </div>

          <div className="hidden md:flex items-center gap-1.5 text-xs text-[var(--neu-muted)] font-bold">
            <span className="text-[var(--neu-teal)]">Sanjay (Lead)</span>
            <span>·</span>
            <span className="text-[var(--neu-orange)]">Kaviya (AI)</span>
          </div>
        </div>
      </header>

      {/* VIEW 1: Phase 2 Architecture & Live Risk Evaluator */}
      {viewMode === 'architecture' && <Phase2Architecture />}

      {/* VIEW 2: Neumorphic Component Kit Showcase (Reference Layout) */}
      {viewMode === 'uikit' && (
        <main className="w-full max-w-5xl grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
          {/* ================= LEFT COLUMN ================= */}
          <div className="flex flex-col gap-6 w-full">
            {/* 1. BTC Sparkline Card */}
            <div className="neu-animate-entry">
              <SparklineCard variant="btc" />
            </div>

            {/* 2. Notification Toggle Row */}
            <div className="neu-animate-entry">
              <NeuCard className="flex items-center justify-between p-4">
                <div className="flex items-center gap-3">
                  <div className="neu-inset-sm h-10 w-10 rounded-xl flex items-center justify-center text-[var(--neu-text)]">
                    <Lock className="h-4 w-4" />
                  </div>
                  <span className="text-sm font-bold text-[var(--neu-text)] tracking-wide">
                    Natification
                  </span>
                </div>
                <NeuSwitch
                  checked={notificationActive}
                  onCheckedChange={setNotificationActive}
                  aria-label="Toggle notifications"
                />
              </NeuCard>
            </div>

            {/* 3. Wide Button */}
            <div className="neu-animate-entry">
              <NeuButton className="w-full h-13 font-bold text-sm tracking-wide">
                Button
              </NeuButton>
            </div>

            {/* 4. Wide Input */}
            <div className="neu-animate-entry">
              <NeuInput placeholder="Input Text" aria-label="Input Text field" />
            </div>

            {/* 5. Clock + Teal Sparkline Card ("Tittle 1") */}
            <div className="neu-animate-entry">
              <SparklineCard variant="teal-clock" />
            </div>

            {/* 6. Ring Chart (Donut with 3 Arcs) */}
            <div className="neu-animate-entry flex justify-center py-2">
              <RingChart />
            </div>
          </div>

          {/* ================= RIGHT COLUMN ================= */}
          <div className="flex flex-col gap-6 w-full">
            {/* 1. Balance Card */}
            <div className="neu-animate-entry">
              <BalanceCard />
            </div>

            {/* 2. Credit Card */}
            <div className="neu-animate-entry">
              <CreditCard />
            </div>

            {/* 3. Mini Controls Block: Switches + Pagination + 2x2 Icon Buttons */}
            <div className="neu-animate-entry">
              <NeuCard className="p-5 flex items-center justify-between gap-4">
                {/* Left Stack: 2 Switches & Pagination Dots */}
                <div className="flex flex-col gap-3.5">
                  <NeuSwitch checked={false} aria-label="Mini switch off" />
                  <NeuSwitch
                    checked={miniSwitchActive}
                    onCheckedChange={setMiniSwitchActive}
                    aria-label="Mini switch on"
                  />
                  <PaginationDots
                    activeIndex={paginationIndex}
                    onChange={setPaginationIndex}
                  />
                </div>

                {/* Right Stack: 2x2 Icon Buttons */}
                <div className="grid grid-cols-2 gap-3">
                  <NeuIconButton icon="x" aria-label="Close" />
                  <NeuIconButton icon="plus" aria-label="Add" />
                  <NeuIconButton icon="menu" aria-label="Menu" />
                  <NeuIconButton icon="back" aria-label="Back" />
                </div>
              </NeuCard>
            </div>

            {/* 4. Heart / Like Buttons Row */}
            <div className="neu-animate-entry flex items-center justify-center gap-6 py-2">
              <NeuLikeButton variant="inset" initialLiked={true} aria-label="Liked heart button" />
              <NeuLikeButton variant="raised" initialLiked={false} aria-label="Unliked heart button" />
            </div>

            {/* 5. Legend Rows & Sliders/Progress Bars */}
            <div className="neu-animate-entry">
              <NeuCard className="p-6">
                <LegendRows />
              </NeuCard>
            </div>
          </div>
        </main>
      )}

      {/* Footer */}
      <footer className="mt-12 text-center text-xs text-[var(--neu-muted)]">
        Multi-Agent AI Payment Security Platform · Team Lead: Sanjay · AI Engineer: Kaviya · Phase 2 Complete
      </footer>
    </div>
  )
}
