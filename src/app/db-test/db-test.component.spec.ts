import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DbTestComponent } from './db-test.component';

describe('DbTestComponent', () => {
  let component: DbTestComponent;
  let fixture: ComponentFixture<DbTestComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DbTestComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DbTestComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
