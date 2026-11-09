import { forwardRef, type HTMLAttributes } from 'react'
import { cn } from '@/lib/utils'

export interface NeuCardProps extends HTMLAttributes<HTMLDivElement> {
  variant?: 'raised' | 'inset' | 'flat'
  size?: 'default' | 'sm'
}

export const NeuCard = forwardRef<HTMLDivElement, NeuCardProps>(
  ({ className, variant = 'raised', size = 'default', children, ...props }, ref) => {
    return (
      <div
        ref={ref}
        className={cn(
          'transition-all duration-200',
          variant === 'raised' && (size === 'sm' ? 'neu-raised-sm' : 'neu-raised'),
          variant === 'inset' && (size === 'sm' ? 'neu-inset-sm' : 'neu-inset'),
          'p-5 text-[var(--neu-text)]',
          className
        )}
        {...props}
      >
        {children}
      </div>
    )
  }
)

NeuCard.displayName = 'NeuCard'
