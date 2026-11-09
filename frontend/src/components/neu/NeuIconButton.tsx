import { forwardRef, type ReactNode } from 'react'
import { X, Plus, Menu, ArrowLeft } from 'lucide-react'
import { NeuButton, type NeuButtonProps } from './NeuButton'
import { cn } from '@/lib/utils'

export type NeuIconVariant = 'x' | 'plus' | 'menu' | 'back'

export interface NeuIconButtonProps extends Omit<NeuButtonProps, 'children'> {
  icon?: NeuIconVariant
  'aria-label': string
  children?: ReactNode
}

export const NeuIconButton = forwardRef<HTMLButtonElement, NeuIconButtonProps>(
  ({ icon, 'aria-label': ariaLabel, children, className, ...props }, ref) => {
    const renderIcon = () => {
      if (children) return children
      switch (icon) {
        case 'x':
          return <X className="h-5 w-5 text-[var(--neu-text)]" />
        case 'plus':
          return <Plus className="h-5 w-5 text-[var(--neu-text)]" />
        case 'menu':
          return <Menu className="h-5 w-5 text-[var(--neu-text)]" />
        case 'back':
          return <ArrowLeft className="h-5 w-5 text-[var(--neu-text)]" />
        default:
          return null
      }
    }

    return (
      <NeuButton
        ref={ref}
        size="icon"
        aria-label={ariaLabel}
        className={cn('rounded-2xl', className)}
        {...props}
      >
        {renderIcon()}
      </NeuButton>
    )
  }
)

NeuIconButton.displayName = 'NeuIconButton'
