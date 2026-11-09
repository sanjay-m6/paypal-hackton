import { useRef, useState } from 'react'
import { Lock } from 'lucide-react'
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

export default function NeumorphismPage() {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const [notificationActive, setNotificationActive] = useState(true)
  const [miniSwitchActive, setMiniSwitchActive] = useState(true)
  const [paginationIndex, setPaginationIndex] = useState(0)

  // GSAP Staggered Entrance Animation with Reduced Motion Support
  useGSAP(
    () => {
      const mm = gsap.matchMedia()

      mm.add('(prefers-reduced-motion: no-preference)', () => {
        gsap.from('.neu-animate-entry', {
          y: 24,
          opacity: 0,
          duration: 0.8,
          stagger: 0.06,
          ease: 'power3.out',
        })
      })

      return () => mm.revert()
    },
    { scope: containerRef }
  )

  return (
    <div
      ref={containerRef}
      className="min-h-screen w-full bg-[var(--neu-bg)] text-[var(--neu-text)] flex flex-col items-center justify-center p-6 md:p-12 selection:bg-[var(--neu-blue)] selection:text-white"
    >
      {/* Header Bar */}
      <header className="w-full max-w-5xl mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-[var(--neu-text)]">
            Neumorphic UI Kit
          </h1>
          <p className="text-xs text-[var(--neu-muted)]">
            Soft UI with GSAP Animations · White/Light Theme · shadcn Primitives
          </p>
        </div>
        <div className="flex items-center gap-2">
          <span className="neu-dot bg-[var(--neu-blue)] text-[var(--neu-blue)]" />
          <span className="text-xs font-semibold text-[var(--neu-muted)]">Active Theme: #e0e5ec</span>
        </div>
      </header>

      {/* Main 2-Column Responsive Layout */}
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

      {/* Footer */}
      <footer className="mt-12 text-center text-xs text-[var(--neu-muted)]">
        Neumorphic Soft UI Kit · GSAP Animated · Verified with shadcn Primitives
      </footer>
    </div>
  )
}
