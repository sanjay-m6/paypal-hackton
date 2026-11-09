import { useRef, useState, type FC, type ChangeEvent } from 'react'
import { cn } from '@/lib/utils'

export interface NeuSliderProps {
  value?: number
  min?: number
  max?: number
  step?: number
  onChange?: (value: number) => void
  showTooltip?: boolean
  tooltipFormat?: (value: number) => string
  className?: string
  'aria-label'?: string
}

export const NeuSlider: FC<NeuSliderProps> = ({
  value = 75,
  min = 0,
  max = 100,
  step = 1,
  onChange,
  showTooltip = true,
  tooltipFormat = (v) => (v / 100 * 1.6).toFixed(3),
  className,
  'aria-label': ariaLabel = 'Slider control',
}) => {
  const [internalValue, setInternalValue] = useState(value)
  const isControlled = onChange !== undefined
  const currentValue = isControlled ? value : internalValue

  const trackRef = useRef<HTMLDivElement | null>(null)
  const percentage = Math.min(100, Math.max(0, ((currentValue - min) / (max - min)) * 100))

  const handleInput = (e: ChangeEvent<HTMLInputElement>) => {
    const nextVal = parseFloat(e.target.value)
    if (!isControlled) {
      setInternalValue(nextVal)
    }
    onChange?.(nextVal)
  }

  return (
    <div className={cn('relative flex flex-col items-center w-full pt-8 pb-2', className)}>
      {/* Floating Tooltip */}
      {showTooltip && (
        <div
          className="absolute -top-1 pointer-events-none transition-all duration-75"
          style={{ left: `calc(${percentage}% - 22px)` }}
        >
          <div className="neu-raised-sm px-2.5 py-1 text-[11px] font-semibold text-[var(--neu-text)] rounded-xl border border-white/80 shadow-sm flex items-center justify-center min-w-[44px]">
            {tooltipFormat(currentValue)}
          </div>
        </div>
      )}

      {/* Track & Input */}
      <div ref={trackRef} className="neu-inset-sm relative h-3.5 w-full rounded-full p-0.5 flex items-center">
        {/* Gradient Fill */}
        <div
          style={{ width: `${percentage}%` }}
          className="h-full rounded-full bg-gradient-to-r from-[var(--neu-orange)] to-[var(--neu-red)] transition-all duration-75"
        />

        {/* Thumb Knob */}
        <div
          style={{ left: `calc(${percentage}% - 10px)` }}
          className="absolute top-1/2 -translate-y-1/2 h-5 w-5 rounded-full bg-[var(--neu-red)] shadow-[0_0_10px_var(--neu-red)] border-2 border-white pointer-events-none transition-all duration-75"
        />

        {/* Hidden Range Input */}
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={currentValue}
          onChange={handleInput}
          aria-label={ariaLabel}
          className="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
        />
      </div>
    </div>
  )
}
