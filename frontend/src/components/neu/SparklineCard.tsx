import { useRef, type FC } from 'react'
import { NeuCard } from './NeuCard'
import { cn } from '@/lib/utils'
import { gsap, useGSAP } from '@/lib/gsap'

export interface SparklineCardProps {
  variant?: 'btc' | 'teal-clock'
  className?: string
}

export const SparklineCard: FC<SparklineCardProps> = ({
  variant = 'btc',
  className,
}) => {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const pathRef = useRef<SVGPathElement | null>(null)

  useGSAP(
    () => {
      if (!pathRef.current) return
      const length = pathRef.current.getTotalLength()
      gsap.fromTo(
        pathRef.current,
        { strokeDasharray: length, strokeDashoffset: length },
        { strokeDashoffset: 0, duration: 1.4, ease: 'power2.inOut' }
      )
    },
    { scope: containerRef }
  )

  if (variant === 'teal-clock') {
    return (
      <NeuCard ref={containerRef} className={cn('flex items-center justify-between p-4', className)}>
        {/* Inset Clock on Left */}
        <div className="flex items-center gap-3.5">
          <div className="neu-inset-sm h-14 w-14 rounded-full flex items-center justify-center relative p-1">
            <svg viewBox="0 0 40 40" className="h-10 w-10 text-[var(--neu-text)]">
              <circle cx="20" cy="20" r="18" fill="none" stroke="rgba(255,255,255,0.4)" strokeWidth="1" />
              {/* Hour hand */}
              <line x1="20" y1="20" x2="14" y2="24" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
              {/* Minute hand */}
              <line x1="20" y1="20" x2="28" y2="22" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
              {/* Center dot */}
              <circle cx="20" cy="20" r="2" fill="currentColor" />
            </svg>
          </div>

          <div>
            <h4 className="text-sm font-bold text-[var(--neu-text)]">Tittle 1</h4>
            <p className="text-[11px] text-[var(--neu-muted)] leading-tight">Tittle 2</p>
            <p className="text-[11px] text-[var(--neu-muted)] leading-tight">Tittle 2</p>
          </div>
        </div>

        {/* Teal Sparkline on Right */}
        <div className="flex flex-col items-end gap-1">
          <svg className="w-28 h-10 overflow-visible" viewBox="0 0 120 40">
            <defs>
              <linearGradient id="tealGrad" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stopColor="var(--neu-green)" />
                <stop offset="100%" stopColor="var(--neu-teal)" />
              </linearGradient>
            </defs>
            <path
              ref={pathRef}
              d="M 5,25 Q 20,35 35,20 T 65,22 T 95,12 T 115,8"
              fill="none"
              stroke="url(#tealGrad)"
              strokeWidth="2.5"
              strokeLinecap="round"
            />
          </svg>
          <div className="flex items-center gap-1.5 text-xs font-semibold text-[var(--neu-text)]">
            <span>Tittle</span>
            <span className="neu-dot bg-[var(--neu-teal)] text-[var(--neu-teal)]" />
          </div>
        </div>
      </NeuCard>
    )
  }

  // BTC Variant (wavy gradient line, yellow -> orange -> red)
  return (
    <NeuCard ref={containerRef} className={cn('relative p-5 overflow-hidden', className)}>
      <div className="h-20 w-full flex items-center justify-center">
        <svg className="w-full h-16 overflow-visible" viewBox="0 0 300 60" preserveAspectRatio="none">
          <defs>
            <linearGradient id="btcGrad" x1="0%" y1="0%" x2="100%" y2="0%">
              <stop offset="0%" stopColor="var(--neu-yellow)" />
              <stop offset="50%" stopColor="var(--neu-orange)" />
              <stop offset="100%" stopColor="var(--neu-red)" />
            </linearGradient>
          </defs>
          <path
            ref={pathRef}
            d="M 10,40 Q 25,20 40,45 T 70,30 T 100,50 T 130,15 T 160,35 T 190,10 T 220,40 T 250,20 T 280,30 T 295,15"
            fill="none"
            stroke="url(#btcGrad)"
            strokeWidth="3"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
      </div>

      <div className="flex items-center justify-end gap-2 mt-1">
        <span className="text-xs font-bold text-[var(--neu-text)] tracking-wider">BTC</span>
        <span className="neu-dot bg-[var(--neu-red)] text-[var(--neu-red)]" />
      </div>
    </NeuCard>
  )
}
