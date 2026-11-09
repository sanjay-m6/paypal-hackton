import { useRef, type FC } from 'react'
import { cn } from '@/lib/utils'
import { gsap, useGSAP } from '@/lib/gsap'

export interface RingChartProps {
  className?: string
}

export const RingChart: FC<RingChartProps> = ({ className }) => {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const arc1Ref = useRef<SVGPathElement | null>(null)
  const arc2Ref = useRef<SVGPathElement | null>(null)
  const arc3Ref = useRef<SVGPathElement | null>(null)

  useGSAP(
    () => {
      const arcs = [arc1Ref.current, arc2Ref.current, arc3Ref.current].filter(Boolean)
      arcs.forEach((arc) => {
        if (!arc) return
        const len = arc.getTotalLength()
        gsap.set(arc, { strokeDasharray: len, strokeDashoffset: len })
      })

      gsap.to(arcs, {
        strokeDashoffset: 0,
        duration: 1.5,
        stagger: 0.2,
        ease: 'power3.out',
      })
    },
    { scope: containerRef }
  )

  // Donut radius R=68, center (100, 100), strokeWidth=16
  // Arc 1: ~180 degrees (from ~45 deg to 225 deg: yellow to orange to red)
  // Arc 2: ~75 degrees (lime green: from ~235 deg to 310 deg)
  // Arc 3: ~65 degrees (teal: from ~320 deg to 25 deg)

  return (
    <div
      ref={containerRef}
      className={cn(
        'neu-raised relative h-52 w-52 rounded-full flex items-center justify-center p-3 select-none',
        className
      )}
    >
      {/* SVG Canvas for 3 Arcs */}
      <svg viewBox="0 0 200 200" className="w-full h-full transform -rotate-90 z-10 overflow-visible">
        <defs>
          {/* Arc 1 Gradient: Yellow to Orange to Red */}
          <linearGradient id="ringGradRed" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="var(--neu-yellow)" />
            <stop offset="50%" stopColor="var(--neu-orange)" />
            <stop offset="100%" stopColor="var(--neu-red)" />
          </linearGradient>

          {/* Arc 2 Gradient: Lime Green */}
          <linearGradient id="ringGradGreen" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="#a0e000" />
            <stop offset="100%" stopColor="var(--neu-green)" />
          </linearGradient>

          {/* Arc 3 Gradient: Teal */}
          <linearGradient id="ringGradTeal" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="var(--neu-green)" />
            <stop offset="100%" stopColor="var(--neu-teal)" />
          </linearGradient>
        </defs>

        {/* Arc 1: Red/Orange large arc (length ~200 deg) */}
        <path
          ref={arc1Ref}
          d="M 148,148 A 68,68 0 1,1 52,148"
          fill="none"
          stroke="url(#ringGradRed)"
          strokeWidth="18"
          strokeLinecap="round"
        />

        {/* Arc 2: Green medium arc */}
        <path
          ref={arc2Ref}
          d="M 45,135 A 68,68 0 0,1 60,55"
          fill="none"
          stroke="url(#ringGradGreen)"
          strokeWidth="18"
          strokeLinecap="round"
        />

        {/* Arc 3: Teal small arc */}
        <path
          ref={arc3Ref}
          d="M 75,45 A 68,68 0 0,1 135,45"
          fill="none"
          stroke="url(#ringGradTeal)"
          strokeWidth="18"
          strokeLinecap="round"
        />
      </svg>

      {/* Inset Hole in Center */}
      <div className="neu-inset absolute h-24 w-24 rounded-full pointer-events-none" />
    </div>
  )
}
