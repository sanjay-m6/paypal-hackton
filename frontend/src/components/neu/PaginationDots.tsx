import { useState, type FC } from 'react'
import { cn } from '@/lib/utils'

export interface PaginationDotsProps {
  total?: number
  activeIndex?: number
  onChange?: (index: number) => void
  className?: string
}

export const PaginationDots: FC<PaginationDotsProps> = ({
  total = 3,
  activeIndex = 0,
  onChange,
  className,
}) => {
  const [current, setCurrent] = useState(activeIndex)

  const handleSelect = (idx: number) => {
    setCurrent(idx)
    onChange?.(idx)
  }

  return (
    <div className={cn('inline-flex items-center gap-2 p-1.5', className)}>
      {Array.from({ length: total }).map((_, idx) => {
        const isActive = idx === (onChange ? activeIndex : current)
        return (
          <button
            key={idx}
            type="button"
            aria-label={`Go to slide ${idx + 1}`}
            onClick={() => handleSelect(idx)}
            className={cn(
              'h-2.5 w-2.5 rounded-full transition-all duration-300 cursor-pointer outline-none',
              'focus-visible:ring-2 focus-visible:ring-[var(--neu-blue)]',
              isActive
                ? 'bg-white shadow-[0_0_8px_rgba(255,255,255,0.9)] scale-110'
                : 'bg-[var(--neu-dark)] opacity-40 hover:opacity-75'
            )}
          />
        )
      })}
    </div>
  )
}
