import { useRef, type FC } from 'react'
import { cn } from '@/lib/utils'
import { gsap, useGSAP } from '@/lib/gsap'

export interface NeuProgressProps {
  value: number // 0 to 100
  variant?: 'red' | 'teal' | 'gradient'
  className?: string
  'aria-label'?: string
}

export const NeuProgress: FC<NeuProgressProps> = ({
  value,
  variant = 'red',
  className,
  'aria-label': ariaLabel = 'Progress bar',
}) => {
  const barRef = useRef<HTMLDivElement | null>(null)
  const containerRef = useRef<HTMLDivElement | null>(null)

  useGSAP(
    () => {
      if (!barRef.current) return
      gsap.fromTo(
        barRef.current,
        { width: '0%' },
        {
          width: `${Math.min(100, Math.max(0, value))}%`,
          duration: 1.2,
          ease: 'power2.out',
        }
      )
    },
    { scope: containerRef, dependencies: [value] }
  )

  const getGradient = () => {
    switch (variant) {
      case 'red':
        return 'bg-gradient-to-r from-[var(--neu-orange)] to-[var(--neu-red)]'
      case 'teal':
        return 'bg-gradient-to-r from-[var(--neu-green)] to-[var(--neu-teal)]'
      case 'gradient':
      default:
        return 'bg-gradient-to-r from-[var(--neu-orange)] via-[var(--neu-yellow)] to-[var(--neu-green)]'
    }
  }

  return (
    <div
      ref={containerRef}
      role="progressbar"
      aria-valuenow={value}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-label={ariaLabel}
      className={cn('neu-inset-sm relative h-3.5 w-full overflow-hidden p-0.5', className)}
    >
      <div
        ref={barRef}
        style={{ width: `${value}%` }}
        className={cn('h-full rounded-full transition-all duration-300', getGradient())}
      />
    </div>
  )
}
