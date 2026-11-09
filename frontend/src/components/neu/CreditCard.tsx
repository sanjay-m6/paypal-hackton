import { type FC } from 'react'
import { NeuCard } from './NeuCard'
import { cn } from '@/lib/utils'

export interface CreditCardProps {
  cardType?: string
  maskedDigits?: string
  expiry?: string
  className?: string
}

export const CreditCard: FC<CreditCardProps> = ({
  cardType = 'VISA',
  maskedDigits = '***   ***   ***   9098',
  expiry = '12/23',
  className,
}) => {
  return (
    <NeuCard
      className={cn(
        'relative overflow-hidden p-6 h-48 flex flex-col justify-between rounded-3xl select-none',
        className
      )}
    >
      {/* Translucent Glass Circles on Right */}
      <div className="absolute -right-10 -top-10 h-44 w-44 rounded-full border border-white/60 bg-white/20 backdrop-blur-sm pointer-events-none" />
      <div className="absolute right-6 -bottom-14 h-40 w-40 rounded-full border border-white/50 bg-white/15 backdrop-blur-sm pointer-events-none" />

      {/* Top Row: VISA Logo */}
      <div className="flex items-center justify-between z-10">
        <span className="text-xl font-black italic tracking-wider text-[var(--neu-text)]">
          {cardType}
        </span>
      </div>

      {/* Middle Row: Masked Digits */}
      <div className="z-10 my-auto">
        <span className="font-mono text-lg font-bold tracking-[0.2em] text-[var(--neu-text)]">
          {maskedDigits}
        </span>
      </div>

      {/* Bottom Row: Expiry */}
      <div className="flex items-center justify-between z-10">
        <span className="text-xs font-semibold text-[var(--neu-muted)] tracking-wider">
          {expiry}
        </span>
      </div>
    </NeuCard>
  )
}
