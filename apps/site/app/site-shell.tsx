import Link from "next/link";
import type { ReactNode } from "react";

export type NavLink = {
  href: string;
  labelJa: string;
  labelEn: string;
};

type SiteShellProps = {
  children: ReactNode;
  footer?: ReactNode;
  navLinks: NavLink[];
  pageClassName?: string;
};

export function SiteShell({
  children,
  footer,
  navLinks,
  pageClassName,
}: SiteShellProps) {
  const pageClass = pageClassName ? `shell ${pageClassName}` : "shell";
  return (
    <main className={pageClass}>
      <nav className={pageClassName ? "subnav" : "nav"}>
        <Link className="brand" href="/">
          DropSquash
        </Link>
        <div className="links">
          {navLinks.map((link) => (
            <Link key={link.href} href={link.href}>
              <span className="nav-copy">
                <span className="nav-ja">{link.labelJa}</span>
                <span className="nav-en">{link.labelEn}</span>
              </span>
            </Link>
          ))}
        </div>
      </nav>
      {children}
      {footer ? <footer>{footer}</footer> : null}
    </main>
  );
}
