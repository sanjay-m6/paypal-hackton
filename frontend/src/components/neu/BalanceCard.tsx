import { useRef, type FC } from 'react'
import { Wallet, ArrowUp } from 'lucide-react'
import { NeuCard } from './NeuCard'
import { cn } from '@/lib/utils'
import { gsap, useGSAP } from '@/lib/gsap'

export interface BalanceCardProps {
  amount?: number
  currency?: string
  deltaAmount?: string
  deltaPercent?: string
  className?: string
}

export const BalanceCard: FC<BalanceCardProps> = ({
  amount = 4567.89,
  currency = '$',
  deltaAmount = '1 098, 23',
  deltaPercent = '34.7%',
  className,
}) => {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const numberRef = useRef<HTMLSpanElement | null>(null)

  useGSAP(
    () => {
      if (!numberRef.current) return
      const targetVal = { val: 0 }
      gsap.to(targetVal, {
        val: amount,
        duration: 1.5,
        ease: 'power3.out',
        onUpdate: () => {
          if (numberRef.current) {
            // Format number to "4 567, 89" format
            const formatted = targetVal.val.toLocaleString('fr-FR', {
              minimumFractionDigits: 2,
              maximumFractionDigits: 2,
            })
            numberRef.current.innerText = `${currency} ${formatted}`
          }
        },
      })
    },
    { scope: containerRef, dependencies: [amount, currency] }
  )

  return (
    <NeuCard ref={containerRef} className={cn('flex items-center justify-between p-6', className)}>
      {/* Wallet Icon */}
      <div className="neu-inset-sm h-14 w-14 rounded-2xl flex items-center justify-center text-[var(--neu-text)]">
        <Wallet className="h-7 w-7 stroke-[1.75]" />
      </div>

      {/* Balance Figures */}
      <div className="flex flex-col items-end">
        <span className="text-xs font-medium text-[var(--neu-muted)] tracking-wide">Balance</span>
        <span ref={numberRef} className="text-2xl font-extrabold text-[var(--neu-text)] tracking-tight">
          {currency} {amount.toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
        </span>
        <div className="flex items-center gap-1 text-xs font-semibold text-[var(--neu-green)] mt-0.5">
          <span>+ {deltaAmount} ({deltaPercent})</span>
          <ArrowUp className="h-3 w-3 stroke-[2.5]" />
        </div>
      </div>
    </NeuCard>
  )
}
