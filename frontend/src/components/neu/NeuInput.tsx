import { forwardRef, type InputHTMLAttributes } from 'react'
import { cn } from '@/lib/utils'

export interface NeuInputProps extends InputHTMLAttributes<HTMLInputElement> {}

export const NeuInput = forwardRef<HTMLInputElement, NeuInputProps>(
  ({ className, type = 'text', ...props }, ref) => {
    return (
      <input
        type={type}
        ref={ref}
        className={cn(
          'neu-inset-sm flex h-12 w-full px-4 text-sm font-normal text-[var(--neu-text)] outline-none transition-all',
          'placeholder:text-[var(--neu-muted)]',
          'focus-visible:ring-2 focus-visible:ring-[var(--neu-blue)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--neu-bg)]',
          'disabled:cursor-not-allowed disabled:opacity-50',
          className
        )}
        {...props}
      />
    )
  }
)

NeuInput.displayName = 'NeuInput'
