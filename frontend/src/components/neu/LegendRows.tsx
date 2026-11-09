import { type FC } from 'react'
import { NeuProgress } from './NeuProgress'
import { NeuSlider } from './NeuSlider'
import { cn } from '@/lib/utils'

export interface LegendRowsProps {
  className?: string
}

export const LegendRows: FC<LegendRowsProps> = ({ className }) => {
  return (
    <div className={cn('flex flex-col gap-4 w-full', className)}>
      {/* Row 1: BTC + Red Progress */}
      <div className="flex items-center gap-3.5">
        <div className="flex items-center gap-2 min-w-[70px]">
          <span className="neu-dot bg-[var(--neu-red)] text-[var(--neu-red)]" />
          <span className="text-xs font-bold tracking-wider text-[var(--neu-text)]">BTC</span>
        </div>
        <div className="flex-1">
          <NeuProgress value={45} variant="red" />
        </div>
      </div>

      {/* Row 2: ETH + Slider with Tooltip (1.235) */}
      <div className="flex items-center gap-3.5">
        <div className="flex items-center gap-2 min-w-[70px]">
          <span className="neu-dot bg-[var(--neu-teal)] text-[var(--neu-teal)]" />
          <span className="text-xs font-bold tracking-wider text-[var(--neu-text)]">ETH</span>
        </div>
        <div className="flex-1">
          <NeuSlider value={78} tooltipFormat={() => '1.235'} />
        </div>
      </div>

      {/* Row 3: LOC + Teal Progress */}
      <div className="flex items-center gap-3.5">
        <div className="flex items-center gap-2 min-w-[70px]">
          <span className="neu-dot bg-[var(--neu-green)] text-[var(--neu-green)]" />
          <span className="text-xs font-bold tracking-wider text-[var(--neu-text)]">LOC</span>
        </div>
        <div className="flex-1">
          <NeuProgress value={62} variant="teal" />
        </div>
      </div>
    </div>
  )
}
